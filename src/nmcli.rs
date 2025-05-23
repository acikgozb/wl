use std::{
    collections::HashMap,
    ffi::OsString,
    fmt,
    io::{self, BufRead, Error, Write},
    os::unix::ffi::OsStringExt,
    process::Command,
};

use crate::{
    adapter::{CARRIAGE_RETURN, Decimal, LINE_FEED, SsidDevPair, Wl},
    api,
};

pub const LOOPBACK_INTERFACE_NAME: &[u8] = b"lo";

pub enum WiFiStatus {
    Enabled,
    Disabled,
}

impl fmt::Display for WiFiStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WiFiStatus::Enabled => write!(f, "enabled"),
            WiFiStatus::Disabled => write!(f, "disabled"),
        }
    }
}

#[derive(Clone)]
pub struct Nmcli;

impl Nmcli {
    pub fn new() -> Self {
        Self
    }

    fn exec(&self, args: &[&[u8]]) -> Result<Vec<u8>, Error> {
        let mut nmcli = Command::new("nmcli");
        let args = args.iter().map(|s| OsString::from_vec(s.to_vec()));
        let cmd = nmcli.args(args).output()?;

        if !cmd.status.success() {
            let nmcli_err = cmd.stderr.lines().collect::<Result<String, Error>>()?;
            return Err(Error::other(nmcli_err));
        }

        Ok(cmd.stdout)
    }
}

impl Wl for Nmcli {
    fn get_wifi_status(&self) -> Result<impl fmt::Display, Error> {
        let args = ["-g", "WIFI", "g"].map(|a| a.as_bytes());
        let result = self.exec(&args)?;

        Ok(if &result[..] == b"enabled\n" {
            WiFiStatus::Enabled
        } else {
            WiFiStatus::Disabled
        })
    }

    fn toggle_wifi(&self) -> Result<impl fmt::Display, Error> {
        let cloned_process = self.clone();
        let prev_status = cloned_process.get_wifi_status()?;

        let mut args = ["radio", "wifi", ""];

        let new_status = if prev_status.to_string() == "enabled" {
            args[2] = "off";
            WiFiStatus::Disabled
        } else {
            args[2] = "on";
            WiFiStatus::Enabled
        };

        let _ = self.exec(&args.map(|a| a.as_bytes()))?;

        Ok(new_status)
    }

    fn get_active_ssid_dev_pairs(&self) -> Result<Vec<SsidDevPair>, Error> {
        let args = ["-g", "NAME,DEVICE", "connection", "show", "--active"];

        let result = self.exec(&args.map(|a| a.as_bytes()))?;

        const NMCLI_FIELD_SEPARATOR: u8 = b':';

        Ok(result
            .split(|b| b == &LINE_FEED)
            .filter_map(|s| {
                let line = s.strip_suffix(&[CARRIAGE_RETURN]).unwrap_or(s);
                if line.is_empty() {
                    None
                } else {
                    let pair = line
                        .split(|b| b == &NMCLI_FIELD_SEPARATOR)
                        .collect::<Vec<&[u8]>>();
                    Some((pair[0].to_vec(), pair[1].to_vec()))
                }
            })
            .collect::<Vec<SsidDevPair>>())
    }

    fn list_networks(&self, show_active: bool, show_ssid: bool) -> Result<(), Error> {
        let mut args = ["", "", "connection", "show", ""];

        if show_ssid {
            args[0] = "--fields";
            args[1] = "NAME";
        }

        if show_active {
            args[4] = "--active";
        }

        let args: Vec<&[u8]> = args
            .into_iter()
            .filter(|a| !a.is_empty())
            .map(|a| a.as_bytes())
            .collect();

        let result = self.exec(&args)?;
        io::stdout().write_all(&result)
    }

    fn get_active_ssids(&self) -> Result<Vec<Vec<u8>>, Error> {
        let args = ["-g", "NAME", "connection", "show", "--active"];

        let result = self.exec(&args.map(|a| a.as_bytes()))?;

        Ok(result
            .split(|b| b == &LINE_FEED)
            .filter_map(|s| {
                let line = s.strip_suffix(&[CARRIAGE_RETURN]).unwrap_or(s);
                if line.is_empty() || line == LOOPBACK_INTERFACE_NAME {
                    return None;
                }

                Some(line.to_vec())
            })
            .collect())
    }

    fn disconnect(&self, ssid: &[u8], forget: bool) -> Result<(), Error> {
        let mut args = [
            "connection",
            if forget { "delete" } else { "down" },
            "id",
            "",
        ]
        .map(|a| a.as_bytes());

        args[3] = ssid;

        let result = self.exec(&args)?;
        io::stdout().write_all(&result[..])
    }

    fn scan(&self, args: &api::ScanArgs) -> Result<Vec<u8>, io::Error> {
        let mut nmcli_args = ["", "", "d", "wifi", "list", "", ""];

        let nmcli_global_args = match (&args.columns, &args.get_values) {
            (None, None) => ["", ""],
            (None, Some(values)) => ["-g", values],
            (Some(columns), None) => ["-f", columns],
            (Some(columns), Some(_)) => ["-f", columns],
        };
        nmcli_args[0..2].copy_from_slice(&nmcli_global_args);

        if args.re_scan {
            nmcli_args[5..].copy_from_slice(&["--rescan", "yes"]);
        }

        let nmcli_args: Vec<&[u8]> = nmcli_args
            .into_iter()
            .filter(|a| !a.is_empty())
            .map(|a| a.as_bytes())
            .collect();

        let scan_result = self.exec(&nmcli_args)?;

        let cloned_process = self.clone();
        let nmcli_args = ["-g", "SIGNAL", "d", "wifi", "list"];

        let signal_result = cloned_process.exec(&nmcli_args.map(|a| a.as_bytes()))?;
        let signal_lines = signal_result
            .split(|b| b == &LINE_FEED)
            .map(|l| l.strip_suffix(&[CARRIAGE_RETURN]).unwrap_or(l))
            .enumerate();

        let mut valid_signals = HashMap::new();

        for (idx, signal) in signal_lines {
            let signal = Decimal::from(signal).inner();

            if signal >= args.min_strength {
                valid_signals.insert(idx + 1, signal);
            }
        }

        let filtered_scan = scan_result
            .split(|b| b == &LINE_FEED)
            .map(|l| l.strip_suffix(&[CARRIAGE_RETURN]).unwrap_or(l))
            .enumerate()
            .filter_map(|(idx, l)| {
                if idx == 0 || valid_signals.contains_key(&idx) {
                    Some([l, &[LINE_FEED]].concat())
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<u8>>();

        Ok(filtered_scan)
    }
}
