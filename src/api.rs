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
    #[clap(visible_alias = "sc")]
    Scan {
        #[command(flatten)]
        args: ScanArgs,
    },

    /// Connect to a WiFi network.
    #[clap(visible_alias = "c")]
    Connect {
        /// SSID to connect.
        ///
        /// If the SSID is not provided, then the program will do
        /// a scan and show the available networks to the user to choose from.
        #[arg(short = 'i', long)]
        ssid: Option<String>,

        /// Re-enter the SSID password even if it is a known network.
        #[arg(short, long, default_value_t = false)]
        force_passwd: bool,
    },

    /// Disconnect from a WiFi network.
    #[clap(visible_alias = "d")]
    Disconnect {
        /// Forget the network (delete it from the known network list).
        #[arg(short, long, default_value_t = false)]
        forget: bool,

        /// SSID of the target network.
        ///
        /// If the SSID is not provided, then the program will show
        /// a list of the connected networks to the user to choose from.
        #[arg(short = 'i', long)]
        ssid: Option<String>,
    },

    /// See known networks.
    #[clap(visible_alias = "ls")]
    ListNetworks {
        /// See active (connected) networks.
        #[arg(short = 'a', long = "active", default_value_t = false)]
        show_active: bool,

        /// Output the SSID's only.
        #[arg(short = 'i', long = "ssid", default_value_t = false)]
        show_ssid: bool,
    },
}

#[derive(clap::Args, Debug)]
pub struct ScanArgs {
    /// Filter scan list based on minimum WiFi signal strength.
    #[arg(short = 's', long, default_value_t = 0)]
    pub min_strength: u8,

    /// Bypass cache and force a re-scan.
    ///
    /// The re-scanning behavior depends on the underlying network backend.
    /// `wl` does not have a custom re-scan.
    #[arg(short = 'r', long, default_value_t = false)]
    pub re_scan: bool,

    /// Only show the specified columns of a network scan.
    ///
    /// This option is useful for filtering the table by column names.
    /// Compared to `--get-values`, the output contains an extra line for
    /// table columns.
    #[arg(short = 'c', long, conflicts_with = "get_values")]
    pub columns: Option<String>,

    /// Show values of specified fields (terse output).
    ///
    /// This option is useful for scripting purposes.
    /// Compared to `--columns`, the output does not contain an extra
    /// line for table columns.
    #[arg(short = 'g', long)]
    pub get_values: Option<String>,
}
