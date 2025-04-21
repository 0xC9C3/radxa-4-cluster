use log::{error, info};

// installs the uf2 by copying the uf2 file to the device
// https://docs.radxa.com/en/x/x4/software/flash?flash_way=Software
pub fn install_uf2() -> Result<(), Box<dyn std::error::Error>> {
    let uf2_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/uf2/build/pwm/pwm_fan.uf2"
    ));

    // execute the script
    let output = std::process::Command::new("/bin/sh")
        .arg("-c")
        .arg("gpioset gpiochip0 17=1 && gpioset gpiochip0 7=1")
        .output()?;

    info!("Script output: {:?}", output);

    // sleep for 1 second
    std::thread::sleep(std::time::Duration::from_secs(1));

    // execute the script
    let output = std::process::Command::new("/bin/sh")
        .arg("-c")
        .arg("gpioset gpiochip0 17=0 && gpioset gpiochip0 7=0")
        .output()?;

    info!("Script output: {:?}", output);

    std::thread::sleep(std::time::Duration::from_secs(3));

    // iterate /dev/disk/by-id and find by name usb-RPI_RP2_* the first partition
    let mut rp2_disk_path = None;
    let disks = std::fs::read_dir("/dev/disk/by-id")?;
    for entry in disks {
        let entry = entry?;
        let path = entry.path();
        if !path.is_symlink() {
            continue;
        }

        if !entry
            .file_name()
            .to_string_lossy()
            .starts_with("usb-RPI_RP2_")
        {
            continue;
        }

        let target = std::fs::canonicalize(&path)?;
        let target_path = target
            .to_str()
            .ok_or("Failed to convert the target path to string")?;

        if target_path.ends_with("1") {
            info!("Found RP2 disk: {:?} - {:?}", &path, target);
            let target = target_path
                .split('/')
                .last()
                .ok_or("Failed to extract the disk path")?;
            let target = format!("/dev/{}", target);

            info!("Link target: {:?}", &target);
            rp2_disk_path = Some(target);
            break;
        }
    }

    if rp2_disk_path.is_none() {
        error!("Disks not found",);
        return Err("RP2 disk not found".into());
    }

    info!("Found RP2 disk: {:?}", rp2_disk_path);

    // mount the disk
    let unix_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    let mount_path = format!("/tmp/rp2_{}", unix_time.as_secs());
    std::fs::create_dir_all(&mount_path)?;
    let output = std::process::Command::new("/bin/sh")
        .arg("-c")
        .arg(format!(
            "mount {} {}",
            rp2_disk_path.ok_or("Failed to convert the target path to string")?,
            mount_path
        ))
        .output()?;

    info!("Mount output: {:?}", output);

    info!("Mounted RP2 disk at {}", &mount_path);

    let uf2_path = format!("{}/pwm_fan.uf2", mount_path);
    std::fs::write(&uf2_path, uf2_bytes)?;

    info!("Wrote uf2 file to {}", &uf2_path);

    Ok(())
}
