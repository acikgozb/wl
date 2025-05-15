mod nmcli;

use std::{error, fmt, io};

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

pub fn toggle() -> Result<(), Error> {
    let current_wifi_status = nmcli::get_wifi_status().map_err(Error::CannotGetWifiStatus)?;
    let toggled_status =
        nmcli::toggle_wifi(current_wifi_status).map_err(Error::CannotToggleWifi)?;

    println!("wifi: {}", toggled_status);

    Ok(())
}

pub fn status() -> Result<(), Error> {
    let active_conns = nmcli::show_active_connections()
        .map_err(Error::CannotGetActiveConnections)?
        .join(", ");

    let wifi_status = nmcli::get_wifi_status().map_err(Error::CannotGetWifiStatus)?;

    println!("wifi: {}", wifi_status);
    println!("connected networks: {}", active_conns);

    Ok(())
}

pub fn list_networks(active: bool, ssid: bool) -> Result<(), Error> {
    let networks = nmcli::show_connections(active, ssid)
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
