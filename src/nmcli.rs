use std::{
    collections::HashMap,
    ffi::OsString,
    io::{self, BufRead},
    os::unix::ffi::OsStringExt,
    process::Command,
};

use crate::{
    adapter::{CARRIAGE_RETURN, Decimal, Error, LINE_FEED, Wl},
    api,
};

#[derive(Clone)]
pub struct Nmcli;

impl Nmcli {
    pub fn new() -> Self {
        Self
    }

    fn exec(&self, args: &[&[u8]]) -> Result<Vec<u8>, (io::Error, i32)> {
        let default_ecode = 1i32;
        let mut nmcli = Command::new("nmcli");
        let args = args.iter().map(|s| OsString::from_vec(s.to_vec()));
        let cmd = nmcli
            .args(args)
            .output()
            .map_err(|err| (err, default_ecode))?;

        if !cmd.status.success() {
            let nmcli_err = cmd
                .stderr
                .lines()
                .collect::<Result<String, io::Error>>()
                .map_err(|err| (err, default_ecode))?;
            let ecode = cmd.status.code().unwrap_or(default_ecode);
            return Err((io::Error::other(nmcli_err), ecode));
        }

        Ok(cmd.stdout)
    }
}

impl Wl for Nmcli {
    fn get_wifi_status(&self) -> Result<Vec<u8>, Error> {
        let args = ["-g", "WIFI", "g"].map(|a| a.as_bytes());
        let result = self.exec(&args).map_err(Error::CannotGetWiFiStatus)?;

        Ok(result
            .split(|a| a == &LINE_FEED)
            .flat_map(|l| l.strip_suffix(&[CARRIAGE_RETURN]).unwrap_or(l))
            .copied()
            .collect())
    }

    fn toggle_wifi(&self) -> Result<Vec<u8>, Error> {
        let cloned_process = self.clone();
        let prev_status = cloned_process.get_wifi_status()?;

        let mut args = ["radio", "wifi", ""];

        let new_status = if &prev_status[..] == b"enabled" {
            args[2] = "off";
            b"disabled".to_vec()
        } else {
            args[2] = "on";
            b"enabled".to_vec()
        };

        let _ = self
            .exec(&args.map(|a| a.as_bytes()))
            .map_err(Error::CannotToggleWiFi)?;

        Ok(new_status)
    }

    fn get_active_ssid_dev_pairs(&self) -> Result<Vec<u8>, Error> {
        let args = ["-g", "NAME,DEVICE", "connection", "show", "--active"];

        self.exec(&args.map(|a| a.as_bytes()))
            .map_err(Error::CannotGetActiveConnections)
    }

    fn list_networks(&self, show_active: bool, show_ssid: bool) -> Result<Vec<u8>, Error> {
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

        self.exec(&args).map_err(Error::CannotListNetworks)
    }

    fn get_active_ssids(&self) -> Result<Vec<u8>, Error> {
        let args = ["-g", "NAME", "connection", "show", "--active"];

        self.exec(&args.map(|a| a.as_bytes()))
            .map_err(Error::CannotGetSSIDStatus)
    }

    fn disconnect(&self, ssid: &[u8], forget: bool) -> Result<Vec<u8>, Error> {
        let mut args = [
            "connection",
            if forget { "delete" } else { "down" },
            "id",
            "",
        ]
        .map(|a| a.as_bytes());
        args[3] = ssid;

        self.exec(&args).map_err(Error::CannotDisconnect)
    }

    fn scan(&self, args: &api::ScanArgs) -> Result<Vec<u8>, Error> {
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

        let scan_result = self.exec(&nmcli_args).map_err(Error::CannotScanWiFi)?;

        let cloned_process = self.clone();
        let nmcli_args = ["-g", "SIGNAL", "d", "wifi", "list"];

        let signal_result = cloned_process
            .exec(&nmcli_args.map(|a| a.as_bytes()))
            .map_err(Error::CannotScanWiFi)?;
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

    fn is_known_ssid(&self, ssid: &[u8]) -> Result<bool, Error> {
        let args = ["-g", "NAME", "connection", "show"].map(|a| a.as_bytes());

        let result = self.exec(&args).map_err(Error::CannotGetSSIDStatus)?;
        let exists = result
            .split(|b| b == &LINE_FEED)
            .map(|l| l.strip_suffix(&[CARRIAGE_RETURN]).unwrap_or(l))
            .any(|l| ssid == l);

        Ok(exists)
    }

    fn connect(
        &self,
        ssid: &[u8],
        passwd: Option<&[u8]>,
        is_known_ssid: bool,
    ) -> Result<Vec<u8>, Error> {
        if is_known_ssid && passwd.is_some() {
            self.disconnect(ssid, true)?;
        }

        let args = if let Some(passwd) = passwd {
            let mut args = ["d", "wifi", "connect", "", "password", ""].map(|a| a.as_bytes());
            args[3] = ssid;
            args[5] = passwd;

            args.to_vec()
        } else {
            let mut args = ["connection", "up", "id", ""].map(|a| a.as_bytes());
            args[3] = ssid;

            args.to_vec()
        };

        self.exec(&args).map_err(Error::CannotConnect)
    }

    fn get_field_separator(&self) -> u8 {
        b':'
    }
}
