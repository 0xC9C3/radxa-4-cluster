use crate::default_curve::get_default_curve;
use clap::Parser;
use log::{debug, error, info};
use std::error::Error;

enum ExitErrorCode {
    FailedToParseSteps,
    NoSerialPorts,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The temperature:fan-speed steps to use i.e. --step 50:100 for 100% fan speed at 50 degrees
    #[arg(short, long)]
    step: Vec<String>,

    /// Install the uf2 file to the device
    #[arg(short, long, default_value_t = false)]
    install: bool,

    /// The serial port to use
    #[arg(short, long, default_value = "/dev/ttyS0")]
    port: String,

    /// The baud rate to use
    #[arg(short, long, default_value_t = 115200)]
    baud: u32,

    /// The port timeout in milliseconds
    #[arg(short, long, default_value_t = 1000)]
    timeout: u32,

    /// refresh rate in milliseconds
    #[arg(short, long, default_value_t = 1000)]
    refresh_rate: u32,

    /// log level
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

impl Args {
    pub fn get_steps(&self) -> Result<Vec<(f32, f32)>, String> {
        let mut steps = Vec::new();
        for step in &self.step {
            let parts: Vec<&str> = step.split(':').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid step format: {}", step));
            }
            let temp: f32 = parts[0]
                .parse()
                .map_err(|_| format!("Invalid temperature value: {}", parts[0]))?;
            let fan_speed: f32 = parts[1]
                .parse()
                .map_err(|_| format!("Invalid fan speed value: {}", parts[1]))?;
            steps.push((temp, fan_speed));
        }
        Ok(steps)
    }

    pub fn get_port(&self) -> &str {
        &self.port
    }

    pub fn get_baud(&self) -> u32 {
        self.baud
    }

    pub fn get_timeout(&self) -> u32 {
        self.timeout
    }

    pub fn get_refresh_rate(&self) -> u32 {
        self.refresh_rate
    }

    pub fn get_install(&self) -> bool {
        self.install
    }

    pub fn initialize(&self) {
        env_logger::Builder::new()
            .filter_level(self.log_level.parse().unwrap_or(log::LevelFilter::Info))
            .init();
    }

    pub fn get_serial_handle(
        &self,
    ) -> Result<(Box<dyn serialport::SerialPort>, Vec<(f32, f32)>), Box<dyn Error>> {
        let target_port = self.get_port();
        let target_port_timeout = self.get_timeout();
        let target_baud = self.get_baud();
        let mut steps = self.get_steps().unwrap_or_else(|err| {
            error!("{}", err);
            std::process::exit(ExitErrorCode::FailedToParseSteps as i32);
        });

        if steps.is_empty() {
            info!("No steps provided, using the default curve.");
            steps = get_default_curve();
        }

        // Sort the steps by temperature
        steps.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // print the steps
        info!("Using Fan curve steps:");
        for (temp, fan_speed) in &steps {
            info!("Temperature: {}°C, Fan Speed: {}%", temp, fan_speed);
        }

        // print available serial ports
        let ports = serialport::available_ports().unwrap_or_else(|err| {
            error!("Error listing serial ports: {}", err);
            std::process::exit(ExitErrorCode::NoSerialPorts as i32);
        });

        if ports.is_empty() {
            error!("No serial ports found.");
            std::process::exit(ExitErrorCode::NoSerialPorts as i32);
        }

        debug!("Available serial ports:");
        for port in &ports {
            debug!("Port: {}", port.port_name);
        }

        // if serial port is not found, exit
        if !ports.iter().any(|p| p.port_name == target_port) {
            error!("Serial port {} not found.", target_port);
            std::process::exit(ExitErrorCode::NoSerialPorts as i32);
        }

        let port = serialport::new(target_port, target_baud)
            .timeout(std::time::Duration::from_millis(target_port_timeout as u64))
            .open()
            .unwrap_or_else(|err| {
                error!("Failed to open serial port: {}", err);
                std::process::exit(ExitErrorCode::NoSerialPorts as i32);
            });

        info!("Serial port {} opened successfully.", target_port);
        Ok((port, steps))
    }
}
