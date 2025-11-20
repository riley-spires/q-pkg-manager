mod cli;
mod config;
mod package;
mod package_manager;

use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use mlua::Lua;

use crate::package::PackageData;

fn main() {
    let lua = Lua::new();
    let cli = Cli::parse();

    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("ERROR: Failed to load config: {}", e);
            exit(1);
        }
    };

    let pkgs = match package::get_packages(&lua, &config) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("ERROR: Failed to retrieve packages: {}", e);
            exit(2);
        }
    };

    match &cli.command {
        Commands::Install => {
            let mut installed_packages = Vec::<PackageData>::new();
            for pkg in pkgs {
                println!("Found pkg: {}", pkg.package_data.name);
                match package_manager::install(&pkg) {
                    Ok(good) => {
                        if good {
                            installed_packages.push(pkg.package_data);
                        }
                    }
                    Err(e) => {
                        eprintln!("ERROR: Failed to install {}: {}", &pkg.package_data.name, e);
                    }
                }

                let file = match File::create(config.config_dir.join("installed_packages.json")) {
                    Ok(f) => Some(f),
                    Err(_) => {
                        eprintln!("WARNING: Failed to create installed packages file. Expect limited functionality");
                        None
                    }
                };

                if let Some(mut file) = file {
                    let json = match serde_json::to_string_pretty(&installed_packages) {
                        Ok(j) => Some(j),
                        Err(_) => {
                            eprintln!("WARNING: Failed to serialize installed packages");
                            None
                        }
                    };

                    if let Some(json) = json {
                        match file.write_all(json.as_bytes()) {
                            Ok(_) => {}
                            Err(_) => {
                                eprintln!("WARNING: Failed to write to installed packages file. Expect limited functionality");
                            }
                        }
                    }
                }
            }
        }
        Commands::List(args) => {
            let pkgs = if args.installed {
                package::get_installed_packages(&config)
            } else {
                pkgs.iter().map(|pkg| pkg.package_data.clone()).collect()
            };

            for pkg in pkgs {
                let version = match pkg.version {
                    Some(v) => v,
                    None => "latest".to_string(),
                };
                let channel = match pkg.channel {
                    Some(c) => c,
                    None => "stable".to_string(),
                };

                println!(
                    "{}: {} - {}/{}",
                    &pkg.name, &pkg.package_type, version, channel
                );
            }
        }
        Commands::Purge => {
            let mut installed_pkgs = package::get_installed_packages(&config);
            let mut uninstalled_pkgs = Vec::<PackageData>::new();

            for pkg_data in &installed_pkgs {
                if pkgs.iter().any(|p| p.package_data == *pkg_data) {
                    continue;
                }
                match package_manager::uninstall(pkg_data) {
                    Ok(b) => {
                        if b {
                            println!("Successfully uninstalled {}", pkg_data.name);
                        } else {
                            eprintln!("Failed to uninstall: {}. Not sure why...", pkg_data.name);
                        }

                        uninstalled_pkgs.push(pkg_data.clone());
                    }
                    Err(e) => {
                        eprintln!("Failed to uninstall {}: {}", pkg_data.name, e);
                    }
                }
            }

            for pkg in uninstalled_pkgs {
                let Some(idx) = installed_pkgs.iter().position(|p| p == &pkg) else {
                    continue;
                };

                installed_pkgs.remove(idx);
            }

            let file = match File::create(config.config_dir.join("installed_packages.json")) {
                Ok(f) => Some(f),
                Err(_) => {
                    eprintln!("WARNING: Failed to create installed packages file. Expect limited functionality");
                    None
                }
            };

            if let Some(mut file) = file {
                let json = match serde_json::to_string_pretty(&installed_pkgs) {
                    Ok(j) => Some(j),
                    Err(_) => {
                        eprintln!("WARNING: Failed to serialize installed packages. Expect limited functionality");
                        None
                    }
                };

                if let Some(json) = json {
                    match file.write_all(json.as_bytes()) {
                        Ok(_) => {}
                        Err(_) => {
                            eprintln!("WARNING: Failed to write to installed packages file. Expect limited functionality");
                        }
                    }
                }
            }
        }
    }
}
