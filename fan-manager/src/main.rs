use crate::control::control;
use crate::uf2::install_uf2;
use clap::Parser;
use log::error;

mod control;
mod default_curve;
mod metrics;
mod metrics_server;
mod param;
mod uf2;

#[tokio::main]
async fn main() {
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

    let control_future = control(port, steps, refresh_rate);
    let metrics_future = metrics_server::run_metrics_server();

    tokio::select!(
        _ = control_future => {
            error!("Control task finished unexpectedly");
        },
        _ = metrics_future => {
            error!("Metrics task finished unexpectedly");
        },
    );
}
