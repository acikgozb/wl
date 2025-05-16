use std::{fmt, io};

pub trait Wl {
    fn get_wifi_status(&self) -> Result<impl fmt::Display, io::Error>;
    fn toggle_wifi(&self, prev_status: &str) -> Result<impl fmt::Display, io::Error>;
    fn list_networks(&self, show_active: bool, show_ssid: bool) -> Result<Vec<String>, io::Error>;
    fn get_active_ssid_dev_pairs(&self) -> Result<Vec<String>, io::Error>;
    fn get_active_ssids(&self) -> Result<Vec<String>, io::Error>;
    fn disconnect(&self, ssid: &str, forget: bool) -> Result<(), io::Error>;
}
