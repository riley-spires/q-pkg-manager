use std::process::Command;

use crate::package::{Package, PackageType};

pub fn install(pkg: &Package) -> Result<bool, std::io::Error> {
    if let Some(func) = &pkg.pre_install {
        println!("Running preinstall script");
        let _ = func.call::<()>(());
    }

    println!("Installing {}", pkg.name);
    match pkg.package_type {
        PackageType::Apt => {
            let mut cmd = Command::new("sudo");

            let mut args: Vec<String> = Vec::from(["apt".to_string(), "install".to_string()]);

            if let Some(version) = &pkg.version {
                args.push(format!("{}={}", &pkg.name, version));
            } else {
                args.push(pkg.name.clone());
            }

            if let Some(_) = &pkg.channel {
                eprintln!("WARNING: Channels are not supported for apt packages.");
                eprintln!("Skipping channel argument");
            }

            cmd.args(args);

            let mut child = cmd.spawn()?;
            let ret_code = child.wait()?;

            if let Some(ret_code) = ret_code.code() {
                return Ok(ret_code == 0);
            } else {
                return Ok(false);
            }
        }
        PackageType::Snap => {
            let mut cmd = Command::new("sudo");
            let mut args: Vec<String> = Vec::from(["snap".to_string(), "install".to_string(), pkg.name.clone()]);
            let mut channel_arg : String = "--channel=".to_string();

            if let Some(version) = &pkg.version {
                channel_arg.push_str(format!("{}/", version).as_str());
            }

            if let Some(channel) = &pkg.channel {
                channel_arg.push_str(channel);
            } else {
                channel_arg.push_str("stable");
            }

            args.push(channel_arg);

            cmd.args(args);

            let mut child = cmd.spawn()?;

            child.wait()?;
        }
    }

    if let Some(func) = &pkg.post_install {
        println!("Running postinstall script");
        let _ = func.call::<()>(());
    }

    Ok(())
}
