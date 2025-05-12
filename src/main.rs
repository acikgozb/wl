use clap::{Parser, Subcommand};

fn main() {
    run().unwrap()
}

fn run() -> Result<(), ()> {
    let args = Args::parse();

    println!("{:?}", args);
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
    Status,

    /// Toggle WiFi on and off.
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
