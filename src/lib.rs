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

pub fn disconnect() -> Result<(), Error> {
    todo!()
}
