use crate::control::control;
use crate::uf2::install_uf2;
use clap::Parser;

mod control;
mod default_curve;
mod param;
mod uf2;

fn main() {
    let args = param::Args::parse();
    args.initialize();

    if args.get_install() {
        install_uf2().expect("Failed to install uf2. Make sure you have the correct permissions.");
        return;
    }

    let (port, steps) = args
        .get_serial_handle()
        .expect("Failed to initialize serial port");

    let refresh_rate = args.get_refresh_rate();
    control(port, steps, refresh_rate);
}
