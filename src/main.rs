mod config;
mod package;

use std::process::exit;

use config::Config;

fn main() {
    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("ERROR: Failed to load config: {}", e);
            exit(1);
        }
    };

    let pkgs = match package::get_packages(&config) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("ERROR: Failed to retrieve packages: {}", e);
            exit(2);
        }
    };

    for pkg in pkgs {
        println!("Found pkg: {}", pkg.name);
    }
}
