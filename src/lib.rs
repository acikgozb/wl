mod adapter;
pub mod api;
mod connect;
mod disconnect;
mod list_networks;
mod nmcli;
mod scan;
mod status;
mod toggle;

use std::{
    error, fmt,
    io::{self, Write},
};

pub use connect::connect;
pub use disconnect::disconnect;
pub use list_networks::list_networks;
pub use scan::scan;
pub use status::status;
pub use toggle::toggle;

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

fn write_out(mut f: impl io::Write, buf: &[u8]) -> Result<(), Error> {
    f.write_all(buf).map_err(Error::CannotWriteStdout)
}
