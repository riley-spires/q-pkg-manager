use serde::Deserialize;

use mlua::{Lua, LuaSerdeExt};
use crate::config::Config;
use std::fs::read_to_string;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageType {
    Apt,
    Snap
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub package_type: PackageType,
    pub version: Option<String>,
    pub channel: Option<String>,
}

pub fn get_packages(config: &Config) -> Result<Vec<Package>, String> {
    let mut packages = Vec::<Package>::new();

    for path in &config.packages {
        let f = match read_to_string(&path) {
            Ok(f) => f,
            Err(_) => {
                return Err(format!("Failed to open {}", &path.as_path().display()));
            }
        };

        let lua = Lua::new();

        let pkg : Package = match lua.load(f).eval() {
            Ok(t) => lua.from_value(t).expect("hehe haha"),
            Err(_) => {
                return Err(format!("Failed to load package from {}", &path.as_path().display()));
            }
        };

        packages.push(pkg);
    }

    Ok(packages)
}
