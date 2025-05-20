use crate::{Error, adapter::Wl};

pub fn list_networks(show_active: bool, show_ssid: bool) -> Result<(), Error> {
    let process = crate::new();
    process
        .list_networks(show_active, show_ssid)
        .map_err(Error::CannotListNetworks)
}
