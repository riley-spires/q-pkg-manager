mod cli;
mod config;
mod package;
mod package_manager;

use std::process::exit;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use mlua::Lua;

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
            for pkg in pkgs {
                println!("Found pkg: {}", pkg.name);
                match package_manager::install(&pkg) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("ERROR: Failed to install {}: {}", &pkg.name, e);
                        exit(3);
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
