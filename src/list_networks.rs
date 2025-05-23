use std::{error, io};

use crate::{
    adapter::{self, Wl},
    write_bytes,
};

pub fn list_networks(show_active: bool, show_ssid: bool) -> Result<(), Box<dyn error::Error>> {
    let process = adapter::new();
    let networks = process.list_networks(show_active, show_ssid)?;

    write_bytes(&mut io::stdout(), &networks)?;
    Ok(())
}
