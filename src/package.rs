use serde::Deserialize;

use mlua::{FromLua, Function, Lua, LuaSerdeExt, Table};
use crate::config::Config;
use std::{fmt::Display, fs::read_to_string};

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageType {
    Apt,
    Snap
}

impl Display for PackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Apt => write!(f, "apt"),
            Self::Snap => write!(f, "snap")
        }
    }
}

pub struct Package {
    pub name: String,
    pub package_type: PackageType,
    pub version: Option<String>,
    pub channel: Option<String>,
    pub pre_install: Option<Function>,
    pub post_install: Option<Function>
}


impl<'lua> FromLua for Package {
    fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
        // let data: PackageData = lua.from_value(value.clone())?;

        let table: Table = Table::from_lua(value.clone(), lua)?;

        let name: String = table.get("name")?;
        let package_type: PackageType = lua.from_value(table.get("package_type")?)?;
        let version: Option<String> = table.get("version")?;
        let channel: Option<String> = table.get("channel")?;
        let pre_install: Option<Function> = table.get("pre_install")?;
        let post_install: Option<Function> = table.get("post_install")?;

        Ok(Self{
            name,
            package_type,
            version,
            channel,
            pre_install: pre_install,
            post_install: post_install
        })
    }
}


pub fn get_packages(lua: &Lua, config: &Config) -> Result<Vec<Package>, String> {
    let mut packages = Vec::<Package>::new();

    for path in &config.packages {
        let f = match read_to_string(&path) {
            Ok(f) => f,
            Err(_) => {
                return Err(format!("Failed to open {}", &path.as_path().display()));
            }
        };

        let pkg : Package = match lua.load(f).eval() {
            Ok(t) => t,
            Err(_) => {
                return Err(format!("Failed to load package from {}", &path.as_path().display()));
            }
        };

        packages.push(pkg);
    }

    Ok(packages)
}
