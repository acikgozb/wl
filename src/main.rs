use std::{error, io, os::unix::ffi::OsStringExt, process::ExitCode};

use clap::Parser;
use wl::api;

// TODO: add err handling and proper exit codes.
fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}

fn run() -> Result<(), Box<dyn error::Error>> {
    let args = api::Args::parse();

    let wl_cmd = args.wl_command.unwrap_or(api::WlCommand::Status);
    match wl_cmd {
        api::WlCommand::Status => wl::status(),
        api::WlCommand::Toggle => wl::toggle(),
        api::WlCommand::Scan { args } => {
            let mut out_buf = io::stdout();
            wl::scan(&mut out_buf, args)
        }
        api::WlCommand::Connect { ssid, force } => wl::connect(),
        api::WlCommand::Disconnect { ssid, forget } => {
            let ssid = ssid.map(OsStringExt::into_vec);
            wl::disconnect(ssid, forget)
        }
        api::WlCommand::ListNetworks {
            show_active,
            show_ssid,
        } => wl::list_networks(show_active, show_ssid),
    }?;

    Ok(())
}
