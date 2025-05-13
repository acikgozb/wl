mod nmcli;

use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    CannotGetActiveConnections(io::Error),
    CannotGetWifiStatus(io::Error),
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "will be implemented")
    }
}

pub fn toggle() -> Result<(), Error> {
    todo!()
}

pub fn status() -> Result<(), Error> {
    let active_conns = nmcli::get_active_connections()
        .map_err(Error::CannotGetActiveConnections)?
        .join(", ");

    let wifi_status = nmcli::get_wifi_status().map_err(Error::CannotGetWifiStatus)?;

    println!("wifi: {}", wifi_status);
    println!("connected networks: {}", active_conns);

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
