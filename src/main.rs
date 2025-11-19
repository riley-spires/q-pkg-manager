mod config;
mod package;
mod package_manager;

use std::process::{exit};

use config::Config;
use mlua::Lua;

fn main() {
    let lua = Lua::new();

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

    for pkg in pkgs {
        println!("Found pkg: {}", pkg.name);
        match package_manager::install(&pkg) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("ERROR: Failed to install {}: {}", &pkg.name, e);
                exit(3);
            }
        }
    }
}
