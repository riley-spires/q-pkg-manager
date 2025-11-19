use std::process::Command;

use crate::package::{Package, PackageType};

pub fn install(pkg: &Package) -> Result<bool, std::io::Error> {
    if let Some(func) = &pkg.pre_install {
        println!("Running preinstall script");
        let _ = func.call::<()>(());
    }

    println!("Installing {}", pkg.package_data.name);
    let ret_code: Option<i32> = match pkg.package_data.package_type {
        PackageType::Apt => {
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

            let mut child = cmd.spawn()?;
            let exit_status = child.wait()?;

            exit_status.code()
        }
        PackageType::Snap => {
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

            let mut child = cmd.spawn()?;

            let exit_status = child.wait()?;

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
