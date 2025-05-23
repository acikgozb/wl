use std::{error, io, process::ExitCode};

use clap::Parser;
use wl::{NetworkAdapterError, api};

const PROGRAM: &str = "wl";

fn main() -> ExitCode {
    which::which("nmcli").expect("The underlying network backend should be installed on the host");

    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{PROGRAM}: {err}");

            if let Some(err) = err.downcast_ref::<wl::NetworkAdapterError>() {
                let ecode = match err {
                    NetworkAdapterError::CannotGetWiFiStatus((_, ecode)) => ecode,
                    NetworkAdapterError::CannotToggleWiFi((_, ecode)) => ecode,

                    NetworkAdapterError::CannotListNetworks((_, ecode)) => ecode,
                    NetworkAdapterError::CannotGetActiveConnections((_, ecode)) => ecode,
                    NetworkAdapterError::CannotGetSSIDStatus((_, ecode)) => ecode,
                    NetworkAdapterError::CannotDisconnect((_, ecode)) => ecode,
                    NetworkAdapterError::CannotScanWiFi((_, ecode)) => ecode,
                    NetworkAdapterError::CannotConnect((_, ecode)) => ecode,
                };

                ExitCode::from(*ecode as u8)
            } else {
                ExitCode::from(1u8)
            }
        }
    }
}

fn run() -> Result<(), Box<dyn error::Error>> {
    let args = api::Args::parse();

    let wl_cmd = args.wl_command.unwrap_or(api::WlCommand::Status);
    match wl_cmd {
        api::WlCommand::Status => wl::status(),
        api::WlCommand::Toggle => wl::toggle(),
        api::WlCommand::Scan { args } => wl::scan(&mut io::stdout(), args),
        api::WlCommand::Connect { ssid, force_passwd } => {
            wl::connect(ssid.map(|i| i.into_bytes()), force_passwd)
        }
        api::WlCommand::Disconnect { ssid, forget } => {
            wl::disconnect(ssid.map(|i| i.into_bytes()), forget)
        }
        api::WlCommand::ListNetworks {
            show_active,
            show_ssid,
        } => wl::list_networks(show_active, show_ssid),
    }?;

    Ok(())
}
