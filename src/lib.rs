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
    let active_conns = process
        .get_active_ssid_dev_pairs()
        .map_err(Error::CannotGetActiveConnections)?
        .join(", ");

    let wifi_status = process
        .get_wifi_status()
        .map_err(Error::CannotGetWifiStatus)?;

    println!("wifi: {}", wifi_status);
    println!("connected networks: {}", active_conns);

    Ok(())
}

pub fn list_networks(show_active: bool, show_ssid: bool) -> Result<(), Error> {
    let process = crate::new();
    let networks = process
        .list_networks(show_active, show_ssid)
        .map_err(Error::CannotListNetworks)?
        .join("\n");

    println!("{}", networks);

    Ok(())
}

pub fn connect() -> Result<(), Error> {
    todo!()
}

pub fn scan() -> Result<(), Error> {
    todo!()
}

pub fn disconnect(ssid: Option<String>, forget: bool) -> Result<(), Error> {
    let ssid = match ssid {
        Some(val) => val,
        None => select_active_ssid()?,
    };

    let process = crate::new();
    process
        .disconnect(&ssid, forget)
        .map_err(Error::CouldNotDisconnect)
}

fn select_active_ssid() -> Result<String, Error> {
    let process = crate::new();
    let active_ssids = process
        .get_active_ssids()
        .map_err(Error::CannotGetActiveConnections)?;

    let mut prompt = String::new();
    let mut conns = HashMap::new();

    for (idx, ssid) in active_ssids
        .into_iter()
        .filter(|c| !c.contains(nmcli::LOOPBACK_INTERFACE_NAME))
        .enumerate()
    {
        prompt = format!("{}({}) {}\n", prompt, idx, ssid);
        conns.insert(idx, ssid);
    }

    let mut answer_buf = String::new();

    print!(
        "Select the SSID you want to disconnect from:\n{}\n> ",
        prompt.trim_end()
    );
    io::stdout().flush().map_err(Error::CouldNotAskSSID)?;

    io::stdin()
        .read_line(&mut answer_buf)
        .map_err(|err| Error::InvalidActiveSSID(Some(err.to_string())))?;

    let answer = answer_buf
        .trim()
        .parse::<usize>()
        .map_err(|err| Error::InvalidActiveSSID(Some(err.to_string())))?;

    conns.remove(&answer).ok_or(Error::InvalidActiveSSID(None))
}
