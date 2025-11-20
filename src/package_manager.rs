use os_info::{get, Type};
use std::process::Command;

use crate::package::{Package, PackageType};

use anyhow::{bail, Context, Result};

pub fn install(pkg: &Package) -> Result<bool> {
    if let Some(func) = &pkg.pre_install {
        println!("Running preinstall script");
        let _ = func.call::<()>(());
    }

    println!("Installing {}", pkg.package_data.name);
    let os = get().os_type();
    let ret_code: Option<i32> = match pkg.package_data.package_type {
        PackageType::Apt => {
            if os != Type::Pop && os != Type::Debian && os != Type::Ubuntu {
                eprintln!("Error: Apt is not supported on non-debian based machines");
                bail!(
                    "Invalid os ({}) for apt package: {}",
                    os,
                    &pkg.package_data.name
                );
            }

            let mut cmd = Command::new("sudo");

            let mut args: Vec<String> = Vec::from(["apt".to_string(), "install".to_string()]);

            if let Some(version) = &pkg.package_data.version {
                args.push(format!("{}={}", &pkg.package_data.name, version));
            } else {
                args.push(pkg.package_data.name.clone());
            }

            if pkg.package_data.channel.is_some() {
                eprintln!("WARNING: Channels are not supported for apt packages.");
                eprintln!("Skipping channel argument");
            }

            cmd.args(args);

            let mut child = cmd.spawn().context("Failed to spawn sudo apt child")?;
            let exit_status = child
                .wait()
                .context("Failed to wait for sudo apt child to finish")?;

            exit_status.code()
        }
        PackageType::Snap => {
            if os != Type::Ubuntu {
                eprintln!("ERROR: Snap is not supported on non-ubuntu machines");
                bail!(
                    "Invalid os ({}) for snap package: {}",
                    os,
                    &pkg.package_data.name
                );
            }

            let mut cmd = Command::new("sudo");
            let mut args: Vec<String> = Vec::from([
                "snap".to_string(),
                "install".to_string(),
                pkg.package_data.name.clone(),
            ]);
            let mut channel_arg: String = "--channel=".to_string();

            if let Some(version) = &pkg.package_data.version {
                channel_arg.push_str(format!("{}/", version).as_str());
            }

            if let Some(channel) = &pkg.package_data.channel {
                channel_arg.push_str(channel);
            } else {
                channel_arg.push_str("stable");
            }

            args.push(channel_arg);

            cmd.args(args);

            let mut child = cmd.spawn().context("Failed to spawn sudo snap child")?;

            let exit_status = child
                .wait()
                .context("Failed to wait for sudo snap child to finish")?;

            exit_status.code()
        }
        PackageType::Brew => {
            if os != Type::Macos {
                eprintln!("ERROR: Brew is not supported on non-mac machines");
                bail!(
                    "Invalid os ({}) for brew package: {}",
                    os,
                    &pkg.package_data.name
                );
            }

            let mut cmd = Command::new("sudo");
            let mut args: Vec<String> = Vec::from(["brew".to_string(), "install".to_string()]);
            let mut version_arg: String = pkg.package_data.name.clone();

            if let Some(version) = &pkg.package_data.version {
                version_arg.push_str(format!("@{}", version).as_str());
            }

            args.push(version_arg);
            cmd.args(args);

            let mut child = cmd.spawn().context("Failed to spawn brew child")?;

            let exit_status = child
                .wait()
                .context("Failed to wait for brew child to finish")?;

            exit_status.code()
        }
    };

    if let Some(func) = &pkg.post_install {
        println!("Running postinstall script");
        let _ = func.call::<()>(());
    }

    if let Some(ret_code) = ret_code {
        Ok(ret_code == 0)
    } else {
        Ok(false)
    }
}
