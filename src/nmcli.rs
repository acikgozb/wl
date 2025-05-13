use std::{
    fmt,
    io::{BufRead, Error},
    process::Command,
};

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

pub fn get_active_connections() -> Result<Vec<String>, Error> {
    let mut nmcli = Command::new("nmcli");
    let cmd = nmcli
        .args(["-g", "NAME,DEVICE"])
        .arg("connection")
        .arg("show")
        .arg("--active")
        .output()?;

    if !cmd.status.success() {
        let nmcli_err = cmd.stderr.lines().collect::<Result<String, Error>>()?;
        return Err(Error::other(nmcli_err));
    }

    let active_conn_device_pairs = cmd.stdout.lines().collect::<Result<Vec<String>, Error>>()?;

    Ok(active_conn_device_pairs)
}

pub fn get_wifi_status() -> Result<WiFiStatus, Error> {
    let mut nmcli = Command::new("nmcli");
    let cmd = nmcli.args(["-g", "WIFI"]).arg("g").output()?;

    if !cmd.status.success() {
        let nmcli_err = cmd.stderr.lines().collect::<Result<String, Error>>()?;
        return Err(Error::other(nmcli_err));
    }

    let status = cmd
        .stdout
        .lines()
        .take(1)
        .collect::<Result<String, Error>>()?;

    Ok(if status == "enabled" {
        WiFiStatus::Enabled
    } else {
        WiFiStatus::Disabled
    })
}

pub fn toggle_wifi(old_status: WiFiStatus) -> Result<WiFiStatus, Error> {
    let mut nmcli = Command::new("nmcli");
    let cmd = nmcli.arg("radio").arg("wifi");

    let new_status = match old_status {
        WiFiStatus::Enabled => {
            cmd.arg("off");
            WiFiStatus::Disabled
        }
        WiFiStatus::Disabled => {
            cmd.arg("on");
            WiFiStatus::Enabled
        }
    };

    let cmd = cmd.output()?;

    if !cmd.status.success() {
        let nmcli_err = cmd.stderr.lines().collect::<Result<String, Error>>()?;
        return Err(Error::other(nmcli_err));
    }

    Ok(new_status)
}
