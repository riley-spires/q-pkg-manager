use mlua::{FromLua, Function, Lua, LuaSerdeExt, Table};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use std::fmt::Display;
use std::fs::{read_to_string, File};
use std::io;

use crate::config::Config;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PackageType {
    Apt,
    Snap,
    Brew,
    Winget,
}

impl Display for PackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Apt => write!(f, "apt"),
            Self::Snap => write!(f, "snap"),
            Self::Brew => write!(f, "brew"),
            Self::Winget => write!(f, "winget"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct PackageData {
    pub name: String,
    pub package_type: PackageType,
    pub version: Option<String>,
    pub channel: Option<String>,
    pub hash: String,
}

#[derive(Clone)]
pub struct Package {
    pub package_data: PackageData,
    pub pre_install: Option<Function>,
    pub post_install: Option<Function>,
}

impl FromLua for Package {
    fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
        let path_wrapper = lua.app_data_ref::<FilePathAppData>().ok_or_else(|| {
            mlua::Error::RuntimeError(
                "Context Error: Don't have filepath to compute hash".to_string(),
            )
        })?;

        let path = &path_wrapper.0;

        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();

        io::copy(&mut file, &mut hasher)?;

        let hash = format!("{:x}", hasher.finalize());

        let table: Table = Table::from_lua(value.clone(), lua)?;

        let name: String = table.get("name")?;
        let package_type: PackageType = lua.from_value(table.get("package_type")?)?;
        let version: Option<String> = table.get("version")?;
        let channel: Option<String> = table.get("channel")?;
        let pre_install: Option<Function> = table.get("pre_install")?;
        let post_install: Option<Function> = table.get("post_install")?;

        Ok(Self {
            package_data: PackageData {
                name,
                package_type,
                version,
                channel,
                hash,
            },
            pre_install,
            post_install,
        })
    }
}

struct FilePathAppData(pub String);

pub fn get_packages(lua: &Lua, config: &Config) -> Result<Vec<Package>, String> {
    let mut packages = Vec::<Package>::new();

    for path in &config.packages {
        let f = match read_to_string(path) {
            Ok(f) => f,
            Err(_) => {
                return Err(format!("Failed to open {}", &path.as_path().display()));
            }
        };

        lua.set_app_data(FilePathAppData(path.display().to_string()));

        let pkg: Package = match lua.load(f).eval() {
            Ok(t) => t,
            Err(_) => {
                return Err(format!(
                    "Failed to load package from {}",
                    &path.as_path().display()
                ));
            }
        };

        lua.remove_app_data::<FilePathAppData>();

        packages.push(pkg);
    }

    Ok(packages)
}

pub fn get_installed_packages(config: &Config) -> Vec<PackageData> {
    let json_raw = match read_to_string(config.config_dir.join("installed_packages.json")) {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    serde_json::from_str(&json_raw).unwrap_or_default()
}
