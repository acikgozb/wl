use std::{
    ffi::OsString,
    fmt,
    io::{self, BufRead, Error, Write},
    os::unix::ffi::OsStringExt,
    process::Command,
};

use crate::adapter::{SsidDevPair, Wl};

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
            .split(|b| b == &0xA)
            .filter_map(|s| {
                let line = s.strip_suffix(&[0xD]).unwrap_or(s);
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

        let result = self.exec(&args[..])?;
        io::stdout().write_all(&result)
    }

    fn get_active_ssids(&self) -> Result<Vec<Vec<u8>>, Error> {
        let args = ["-g", "NAME", "connection", "show", "--active"];

        let result = self.exec(&args.map(|a| a.as_bytes()))?;

        Ok(result
            .split(|b| b == &0xA)
            .filter_map(|s| {
                let line = s.strip_suffix(&[0xD]).unwrap_or(s);
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
}
