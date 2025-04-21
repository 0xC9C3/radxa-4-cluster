use log::info;
use sysinfo::{Components, System};

pub fn control(
    mut port: Box<dyn serialport::SerialPort>,
    steps: Vec<(f32, f32)>,
    refresh_rate: u32,
) {
    let mut sys = System::new_all();
    let mut average_temp_collection = Vec::new();
    let mut current_fan_speed = 0f32;
    info!("Starting fan control...");

    loop {
        // Read the temperature from the system and adjust the fan speed accordingly
        sys.refresh_all();

        let components = Components::new_with_refreshed_list();
        let (current_highest_temp, component) = components
            .iter()
            .filter_map(|component| {
                if let Some(temp) = component.temperature() {
                    Some((temp, Some(component)))
                } else {
                    None
                }
            })
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or((0f32, None));

        // collect the average temperature over the last 5 times
        average_temp_collection.push(current_highest_temp);
        if average_temp_collection.len() > 5 {
            average_temp_collection.remove(0);
        }
        let average_temp: f32 =
            average_temp_collection.iter().sum::<f32>() / average_temp_collection.len() as f32;

        let mut fan_speed: f32 = 0f32;
        for (temp_threshold, speed) in &steps {
            if average_temp >= *temp_threshold {
                fan_speed = *speed;
            } else {
                break;
            }
        }

        // If the fan speed is the same as the current one, skip sending the command
        if fan_speed == current_fan_speed {
            std::thread::sleep(std::time::Duration::from_millis(refresh_rate as u64));
            continue;
        }

        // Send the fan speed command to the serial port
        info!(
            "Setting fan speed to {} from {} for temperature {} because of {}",
            fan_speed,
            current_fan_speed,
            current_highest_temp,
            component
                .map(|c| c.label())
                .unwrap_or_else(|| "Unknown component".into())
        );

        current_fan_speed = fan_speed;

        // the speed will be read via atof
        let command = format!("{}\n", fan_speed);
        port.write(command.as_bytes()).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(refresh_rate as u64));
    }
}
