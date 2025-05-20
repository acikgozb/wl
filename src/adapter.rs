use std::{fmt, io};

use crate::api::ScanArgs;

pub type SsidDevPair = (Vec<u8>, Vec<u8>);

pub trait Wl {
    fn get_wifi_status(&self) -> Result<impl fmt::Display, io::Error>;
    fn toggle_wifi(&self) -> Result<impl fmt::Display, io::Error>;
    fn list_networks(&self, show_active: bool, show_ssid: bool) -> Result<(), io::Error>;
    fn get_active_ssid_dev_pairs(&self) -> Result<Vec<SsidDevPair>, io::Error>;
    fn get_active_ssids(&self) -> Result<Vec<Vec<u8>>, io::Error>;
    fn disconnect(&self, ssid: &[u8], forget: bool) -> Result<(), io::Error>;
    fn scan(&self, args: &ScanArgs) -> Result<Vec<u8>, io::Error>;
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
