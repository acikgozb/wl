mod adapter;
mod nmcli;

use std::{
    collections::HashMap,
    error, fmt,
    io::{self, Write},
};

use adapter::Wl;

#[derive(Debug)]
pub enum Error {
    CannotGetActiveConnections(io::Error),
    CannotGetWifiStatus(io::Error),
    CannotToggleWifi(io::Error),
    CannotListNetworks(io::Error),
    InvalidActiveSSID(Option<String>),
    CouldNotAskSSID(io::Error),
    CouldNotDisconnect(io::Error),
    CannotWriteStdout(io::Error),
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "will be implemented")
    }
}

pub fn new() -> impl adapter::Wl {
    nmcli::Nmcli::new()
}

pub fn toggle() -> Result<(), Error> {
    let process = crate::new();
    let prev_status = process
        .get_wifi_status()
        .map_err(Error::CannotGetWifiStatus)?
        .to_string();

    let process = crate::new();
    let toggled_status = process
        .toggle_wifi(prev_status.as_str())
        .map_err(Error::CannotToggleWifi)?;

    println!("wifi: {}", toggled_status);

    Ok(())
}

pub fn status() -> Result<(), Error> {
    let process = crate::new();
    let pairs = process
        .get_active_ssid_dev_pairs()
        .map_err(Error::CannotGetActiveConnections)?;

    let wifi_status = process
        .get_wifi_status()
        .map_err(Error::CannotGetWifiStatus)?;

    let out_buf = format!("wifi: {}\n", wifi_status);
    write_out(out_buf.as_bytes())?;

    let mut out_buf: Vec<u8> = b"connected networks: ".to_vec();
    for (ssid, dev) in pairs {
        let mut pair = [&ssid[..], b"/", &dev[..], b", "].concat();
        out_buf.append(&mut pair);
    }
    write_out(out_buf.strip_suffix(b", ").unwrap())?;

    Ok(())
}

pub fn list_networks(show_active: bool, show_ssid: bool) -> Result<(), Error> {
    let process = crate::new();
    process
        .list_networks(show_active, show_ssid)
        .map_err(Error::CannotListNetworks)
}

pub fn connect() -> Result<(), Error> {
    todo!()
}

pub fn scan() -> Result<(), Error> {
    todo!()
}

pub fn disconnect(ssid: Option<Vec<u8>>, forget: bool) -> Result<(), Error> {
    let ssid = match ssid {
        Some(val) => val,
        None => select_active_ssid()?,
    };

    let process = crate::new();
    process
        .disconnect(&ssid, forget)
        .map_err(Error::CouldNotDisconnect)
}

fn select_active_ssid() -> Result<Vec<u8>, Error> {
    let process = crate::new();
    let active_ssids = process
        .get_active_ssids()
        .map_err(Error::CannotGetActiveConnections)?;

    let mut ssids = Vec::new();
    let mut conns = HashMap::new();

    for (idx, ssid) in active_ssids.into_iter().enumerate() {
        ssids = [
            &ssids[..],
            b"(",
            idx.to_string().as_bytes(),
            b") ",
            &ssid[..],
        ]
        .concat();
        conns.insert(idx, ssid);
    }

    let out_buf = &[
        b"Select the SSID you want to disconnect from:\n",
        &ssids[..],
        b"\n> ",
    ]
    .concat();
    write_out(out_buf)?;
    io::stdout().flush().map_err(Error::CouldNotAskSSID)?;

    let mut answer_buf = String::new();
    io::stdin()
        .read_line(&mut answer_buf)
        .map_err(|err| Error::InvalidActiveSSID(Some(err.to_string())))?;

    let answer = answer_buf
        .trim()
        .parse::<usize>()
        .map_err(|err| Error::InvalidActiveSSID(Some(err.to_string())))?;

    conns.remove(&answer).ok_or(Error::InvalidActiveSSID(None))
}

fn write_out(buf: &[u8]) -> Result<(), Error> {
    io::stdout()
        .write_all(buf)
        .map_err(Error::CannotWriteStdout)
}
