use std::io;

use crate::{Error, adapter::Wl, write_out};

pub fn toggle() -> Result<(), Error> {
    let process = crate::new();
    let toggled_status = process.toggle_wifi().map_err(Error::CannotToggleWifi)?;

    let out_buf = format!("wifi: {}\n", toggled_status);
    write_out(&mut io::stdout(), out_buf.as_bytes())?;

    Ok(())
}
