use std::{error, fmt, io};

use crate::{api::ScanArgs, nmcli};

pub const LINE_FEED: u8 = 0xA;
pub const CARRIAGE_RETURN: u8 = 0xD;

pub type SsidDevPair = (Vec<u8>, Vec<u8>);

pub trait Wl {
    fn get_field_separator(&self) -> u8;
    fn get_wifi_status(&self) -> Result<Vec<u8>, Error>;
    fn toggle_wifi(&self) -> Result<Vec<u8>, Error>;
    fn list_networks(&self, show_active: bool, show_ssid: bool) -> Result<Vec<u8>, Error>;
    fn get_active_ssid_dev_pairs(&self) -> Result<Vec<SsidDevPair>, Error>;
    fn get_active_ssids(&self) -> Result<Vec<Vec<u8>>, Error>;
    fn disconnect(&self, ssid: &[u8], forget: bool) -> Result<Vec<u8>, Error>;
    fn scan(&self, args: &ScanArgs) -> Result<Vec<u8>, Error>;
    fn is_known_ssid(&self, ssid: &[u8]) -> Result<bool, Error>;
    fn connect(
        &self,
        ssid: &[u8],
        passwd: Option<&[u8]>,
        is_known_ssid: bool,
    ) -> Result<Vec<u8>, Error>;
}

pub fn new() -> impl Wl {
    nmcli::Nmcli::new()
}

#[derive(Debug)]
pub enum Error {
    CannotGetWiFiStatus((io::Error, i32)),
    CannotToggleWiFi((io::Error, i32)),
    CannotListNetworks((io::Error, i32)),
    CannotGetActiveConnections((io::Error, i32)),
    CannotGetSSIDStatus((io::Error, i32)),
    CannotDisconnect((io::Error, i32)),
    CannotScanWiFi((io::Error, i32)),
    CannotConnect((io::Error, i32)),
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CannotGetWiFiStatus((err, _)) => {
                write!(f, "unable to get the WiFi status: {}", err)
            }
            Error::CannotToggleWiFi((err, _)) => write!(f, "unable to toggle WiFi: {}", err),
            Error::CannotListNetworks((err, _)) => {
                write!(f, "unable to list the networks: {}", err)
            }
            Error::CannotGetActiveConnections((err, _)) => {
                write!(f, "unable to get the active connections: {}", err)
            }
            Error::CannotGetSSIDStatus((err, _)) => {
                write!(f, "unable to get the SSID status: {}", err)
            }
            Error::CannotDisconnect((err, _)) => write!(f, "unable to disconnect: {}", err),
            Error::CannotScanWiFi((err, _)) => {
                write!(f, "unable to scan the available networks: {}", err)
            }
            Error::CannotConnect((err, _)) => {
                write!(f, "unable to connect to the network: {}", err)
            }
        }
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
