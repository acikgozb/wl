use std::{error, io};

use crate::{
    adapter:: Wl,
    write_bytes,
};

pub fn toggle() -> Result<(), Box<dyn error::Error>> {
    let process = crate::new();
    let toggled_status = process.toggle_wifi()?;

    let out_buf = [b"wifi: ", &toggled_status[..], b" \n"].concat();
    write_bytes(&mut io::stdout(), &out_buf)?;

    Ok(())
}
