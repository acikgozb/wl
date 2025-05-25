use std::{error, io};

use crate::{
    adapter::{self, Wl},
    write_bytes,
};

/// Toggles the WiFi status by using a [`Wl`] implementation.
///
/// The latest WiFi status is written to the stdout stream.
///
/// The format of the WiFi status depends on the [`Wl`] implementation.
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
pub fn toggle() -> Result<(), Box<dyn error::Error>> {
    let process = adapter::new();
    let toggled_status = process.toggle_wifi()?;

    let out_buf = [b"wifi: ", &toggled_status[..], b" \n"].concat();
    write_bytes(&mut io::stdout(), &out_buf)?;

    Ok(())
}
