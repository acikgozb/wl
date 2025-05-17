use std::{
    fmt,
    io::{self, BufRead, Error, Write},
    process::Command,
};

use crate::adapter::Wl;

pub const LOOPBACK_INTERFACE_NAME: &str = "lo";

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

pub struct Nmcli;

impl Nmcli {
    pub fn new() -> Self {
        Self
    }

    fn exec(&self, args: &[&str]) -> Result<Vec<u8>, Error> {
        let mut nmcli = Command::new("nmcli");
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
        let args: [&str; 3] = ["-g", "WIFI", "g"];
        let result = self.exec(&args)?;

        let status = result.lines().take(1).collect::<Result<String, Error>>()?;

        Ok(if status == "enabled" {
            WiFiStatus::Enabled
        } else {
            WiFiStatus::Disabled
        })
    }

    fn toggle_wifi(&self, prev_status: &str) -> Result<impl fmt::Display, Error> {
        let mut args: [&str; 3] = ["radio", "wifi", ""];

        let new_status = if prev_status == "enabled" {
            args[2] = "off";
            WiFiStatus::Disabled
        } else {
            args[2] = "on";
            WiFiStatus::Enabled
        };

        let _ = self.exec(&args)?;

        Ok(new_status)
    }

    // TODO: Return iter of tuples, which allows the caller to decide how to use it.
    fn get_active_ssid_dev_pairs(&self) -> Result<Vec<String>, Error> {
        let args: [&str; 5] = ["-g", "NAME,DEVICE", "connection", "show", "--active"];

        let result = self.exec(&args)?;

        let active_ssid_dev_pairs = result.lines().collect::<Result<Vec<String>, Error>>()?;
        Ok(active_ssid_dev_pairs)
    }

    fn list_networks(&self, show_active: bool, show_ssid: bool) -> Result<(), Error> {
        let mut args: [&str; 5] = ["", "", "connection", "show", ""];

        if show_ssid {
            args[0] = "--fields";
            args[1] = "NAME";
        }

        if show_active {
            args[4] = "--active";
        }

        let args: Vec<&str> = args.into_iter().filter(|a| !a.is_empty()).collect();

        let result = self.exec(&args[..])?;
        io::stdout().write_all(&result)
    }

    fn get_active_ssids(&self) -> Result<Vec<String>, Error> {
        let args: [&str; 5] = ["-g", "NAME", "connection", "show", "--active"];

        let result = self.exec(&args)?;

        result
            .lines()
            .filter(|l| match l {
                Ok(l) => !l.contains(LOOPBACK_INTERFACE_NAME),
                Err(_) => true,
            })
            .collect()
    }

    fn disconnect(&self, ssid: &str, forget: bool) -> Result<(), Error> {
        let mut args: [&str; 4] = ["connection", "", "id", ssid];
        args[1] = if forget { "delete" } else { "down" };

        let result = self.exec(&args)?;
        io::stdout().write_all(&result[..])
    }
}
