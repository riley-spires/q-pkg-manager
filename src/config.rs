use dirs::config_dir;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::env::consts::OS;
use std::fs::create_dir;

pub struct Config {
    pub packages: Vec<PathBuf> 
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let config_dir = if let Some(path) = config_dir() {
            path.join("q-pkg-manager")
        } else {
            match OS {
                "linux" => {
                    Path::new("/opt/q-pkg-manager").to_path_buf()
                },
                _ => todo!()
            }
        };

        if !config_dir.exists() {
            if let Err(_) = create_dir(&config_dir) {
                return Err("Failed to create config dir".to_string());
            }
        }

        if !config_dir.join("packages").exists() {
            if let Err(_) = create_dir(&config_dir.join("packages")) {
                return Err("Failed to create packages dir".to_string())
            }
        }

        let packages : Vec<PathBuf> = WalkDir::new(config_dir.join("packages")).into_iter().skip(1).filter_map(|e| e.ok()).map(|e| PathBuf::from(e.path())).collect();

        Ok(Self{
            packages: packages
        })
    }
}

