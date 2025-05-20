use std::ffi::OsString;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub wl_command: Option<WlCommand>,
}

#[derive(Debug, Subcommand)]
pub enum WlCommand {
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
        ssid: Option<OsString>,

        #[command(flatten)]
        scan_args: ScanArgs,

        /// Re-enter the SSID password even if it is a known network.
        #[arg(short, long, default_value_t = false)]
        force: bool,
    },

    /// Disconnect from a WiFi network.
    #[clap(visible_alias = "d")]
    Disconnect {
        /// Forget the network (delete it from the known network list).
        #[arg(short = 'd', long, default_value_t = false)]
        forget: bool,

        /// SSID of the target network.
        ssid: Option<OsString>,
    },

    /// See known networks.
    #[clap(visible_alias = "ls")]
    ListNetworks {
        /// See active (connected) networks.
        #[arg(short = 'a', long = "active", default_value_t = false)]
        show_active: bool,

        /// Output the SSID's only.
        #[arg(short = 's', long = "ssid", default_value_t = false)]
        show_ssid: bool,
    },
}

#[derive(clap::Args, Debug)]
pub struct ScanArgs {
    /// Filter scan list based on minimum WiFi signal strength (1 to 100).
    #[arg(short = 's', long, default_value_t = 40)]
    min_strength: u8,

    /// Bypass cache and force a re-scan.
    #[arg(short = 'r', long, default_value_t = false)]
    re_scan: bool,

    /// Show specified fields only.
    #[arg(short = 'f', long)]
    fields: Option<String>,

    /// Show values of specified fields (terse output).
    #[arg(short = 'g', long)]
    get_values: Option<String>,
}
