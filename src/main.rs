mod cli;
mod config;
mod package;
mod package_manager;

use std::process::exit;
use std::fs::{File};
use std::io::prelude::*;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use mlua::Lua;

use crate::package::Package;

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
            let mut installed_packages = Vec::<Package>::new();
            for pkg in pkgs {
                println!("Found pkg: {}", pkg.name);
                match package_manager::install(&pkg) {
                    Ok(good) => {
                        if good {
                            installed_packages.push(pkg);
                        } 
                    }
                    Err(e) => {
                        eprintln!("ERROR: Failed to install {}: {}", &pkg.name, e);
                        exit(3);
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
                            Ok(_) => {},
                            Err(_) => {
                                eprintln!("WARNING: Failed to write to installed packages file. Expect limited functionality");
                            }
                        }
                    }
                }
            }
        }
        Commands::List => {
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
    }
}
