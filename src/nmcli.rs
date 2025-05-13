use std::{
    io::{BufRead, Error},
    process::Command,
};

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

pub fn get_wifi_status() -> Result<String, Error> {
    let mut nmcli = Command::new("nmcli");
    let cmd = nmcli.args(["-g", "WIFI"]).arg("g").output()?;

    if !cmd.status.success() {
        let nmcli_err = cmd.stderr.lines().collect::<Result<String, Error>>()?;
        return Err(Error::other(nmcli_err));
    }

    cmd.stdout
        .lines()
        .take(1)
        .collect::<Result<String, Error>>()
}
