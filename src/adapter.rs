use std::{fmt, io};

pub type SsidDevPair = (Vec<u8>, Vec<u8>);

pub trait Wl {
    fn get_wifi_status(&self) -> Result<impl fmt::Display, io::Error>;
    fn toggle_wifi(&self) -> Result<impl fmt::Display, io::Error>;
    fn list_networks(&self, show_active: bool, show_ssid: bool) -> Result<(), io::Error>;
    fn get_active_ssid_dev_pairs(&self) -> Result<Vec<SsidDevPair>, io::Error>;
    fn get_active_ssids(&self) -> Result<Vec<Vec<u8>>, io::Error>;
    fn disconnect(&self, ssid: &[u8], forget: bool) -> Result<(), io::Error>;
}
