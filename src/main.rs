mod config;
mod package;

use std::process::exit;

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
        println!("Running preinstall script");

        if let Some(func) = pkg.pre_install {
            let _ = func.call::<()>(());
        }
    }
}
