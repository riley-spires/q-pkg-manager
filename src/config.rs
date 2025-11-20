use dirs::config_dir;
use std::env::consts::OS;
use std::fs::create_dir;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Config {
    pub packages: Vec<PathBuf>,
    pub config_dir: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let config_dir = if let Some(path) = config_dir() {
            path.join("q-pkg-manager")
        } else {
            match OS {
                "linux" => Path::new("/opt/q-pkg-manager").to_path_buf(),
                _ => todo!(),
            }
        };

        if !config_dir.exists() && create_dir(&config_dir).is_err() {
            return Err("Failed to create config dir".to_string());
        }

        if !config_dir.join("packages").exists() && create_dir(config_dir.join("packages")).is_err()
        {
            return Err("Failed to create packages dir".to_string());
        }

        let packages: Vec<PathBuf> = WalkDir::new(config_dir.join("packages"))
            .into_iter()
            .skip(1)
            .filter_map(|e| e.ok())
            .map(|e| PathBuf::from(e.path()))
            .filter(|p| {
                if p.extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    == "lua"
                {
                    true
                } else {
                    eprintln!(
                        "WARNING: Found non-lua file {} in packages directory. Ignoring it.",
                        p.display()
                    );
                    false
                }
            })
            .collect();

        Ok(Self {
            packages,
            config_dir,
        })
    }
}
