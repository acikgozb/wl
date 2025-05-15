use clap::{Parser, Subcommand};
use std::{error, process::ExitCode};

// TODO: add err handling and proper exit codes.
fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}

fn run() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    let wl_cmd = args.wl_command.unwrap_or(WlCommand::Status);
    match wl_cmd {
        WlCommand::Status => wl::status(),
        WlCommand::Toggle => wl::toggle(),
        WlCommand::Scan(scan_args) => wl::scan(),
        WlCommand::Connect {
            ssid,
            scan_args,
            force,
        } => wl::connect(),
        WlCommand::Disconnect { forget } => wl::disconnect(),
        WlCommand::ListNetworks { active, ssid } => wl::list_networks(active, ssid),
    }?;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    wl_command: Option<WlCommand>,
}

#[derive(Debug, Subcommand)]
enum WlCommand {
    /// Show the overall status of WiFi (on/off, connected network if any)
    #[clap(visible_alias = "s")]
    Status,

    /// Toggle WiFi on and off.
    #[clap(visible_alias = "t")]
    Toggle,

    /// See available WiFi networks.
    Scan(ScanArgs),

    /// Connect to a WiFi network.
    Connect {
        //// SSID to connect.
        ssid: Option<String>,

        #[command(flatten)]
        scan_args: ScanArgs,

        /// Re-enter the SSID password even if it is a known network.
        #[arg(short, long, default_value_t = false)]
        force: bool,
    },

    /// Disconnect from the currently connected WiFi network.
    Disconnect {
        /// Forget the network (delete it from the known network list).
        #[arg(short = 'd', long, default_value_t = false)]
        forget: bool,
    },

    /// See known networks.
    #[clap(visible_alias = "ls")]
    ListNetworks {
        /// See active (connected) networks.
        #[arg(short, long, default_value_t = false)]
        active: bool,

        /// Output the SSID's only.
        #[arg(long, default_value_t = false)]
        ssid: bool,
    },
}

#[derive(clap::Args, Debug)]
struct ScanArgs {
    /// Filter scan list based on minimum scan (1 to 4).
    #[arg(short = 's', long, default_value_t = 2)]
    min_strength: u8,

    /// Turn on re-scanning after a successful scan.
    #[arg(short = 'r', long, default_value_t = false)]
    re_scan: bool,

    /// Set the re-scan refresh timer
    #[arg(short = 't', long, default_value_t = 5)]
    refresh_in: u8,
}
