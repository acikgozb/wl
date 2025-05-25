use std::{error, io};

use crate::{
    adapter::{self, Wl},
    write_bytes,
};

/// Provides the list of known WiFi networks by using a [`Wl`] implementation.
/// To see the available networks to connect, please refer to [`scan`] instead.
///
/// The list is written to stdout stream.
///
/// The default list format depends on the [`Wl`] implementation.
/// If desired, the list can be filtered in two different ways:
///
/// - By showing the SSIDs only,
/// - By showing the active network connections only.
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function can return an [`adapter::Error`] when the underlying [`Wl`] implementation fails or [`io::Error`] when the information cannot be written on the stdout stream.
///
/// [`adapter::Error`]: crate::adapter::Error
/// [`io::Error`]: std::io::Error
/// [`scan`]: crate::scan
pub fn list_networks(show_active: bool, show_ssid: bool) -> Result<(), Box<dyn error::Error>> {
    let process = adapter::new();
    let networks = process.list_networks(show_active, show_ssid)?;

    write_bytes(&mut io::stdout(), &networks)?;
    Ok(())
}
