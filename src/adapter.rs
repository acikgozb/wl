use std::{error, fmt, io};

use crate::{api::ScanArgs, nmcli};

pub const LINE_FEED: u8 = 0xA;
pub const CARRIAGE_RETURN: u8 = 0xD;

pub type SsidDevPair = (Vec<u8>, Vec<u8>);

pub trait Wl {
    fn get_wifi_status(&self) -> Result<Vec<u8>, Error>;
    fn toggle_wifi(&self) -> Result<Vec<u8>, Error>;
    fn list_networks(&self, show_active: bool, show_ssid: bool) -> Result<Vec<u8>, Error>;
    fn get_active_ssid_dev_pairs(&self) -> Result<Vec<SsidDevPair>, Error>;
    fn get_active_ssids(&self) -> Result<Vec<Vec<u8>>, Error>;
    fn disconnect(&self, ssid: &[u8], forget: bool) -> Result<Vec<u8>, Error>;
    fn scan(&self, args: &ScanArgs) -> Result<Vec<u8>, Error>;
}

#[derive(Debug)]
pub enum Error {
    CannotGetWiFiStatus(io::Error),
    CannotToggleWiFi(io::Error),
    CannotListNetworks(io::Error),
    CannotGetActiveConnections(io::Error),
    CannotGetSSIDStatus(io::Error),
    CannotDisconnect(io::Error),
    CannotScanWiFi(io::Error),
    CannotConnect(io::Error),
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "will be implemented")
    }
}

pub struct Decimal(u8);

impl From<&[u8]> for Decimal {
    fn from(value: &[u8]) -> Self {
        Self(value.iter().fold(0, |acc, b| acc * 10 + (b - b'0')))
    }
}

impl Decimal {
    pub fn inner(&self) -> u8 {
        self.0
    }
}
