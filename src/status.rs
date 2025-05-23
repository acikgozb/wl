use std::{error, io};

use crate::{
    adapter::{self, Wl},
    write_bytes,
};

pub fn status() -> Result<(), Box<dyn error::Error>> {
    let process = adapter::new();
    let pairs = process.get_active_ssid_dev_pairs()?;

    let wifi_status = process.get_wifi_status()?;

    let mut stdout = io::stdout();

    let out_buf = [b"wifi: ", &wifi_status[..], b" \n"].concat();
    write_bytes(&mut stdout, &out_buf)?;

    let mut out_buf: Vec<u8> = b"connected networks: ".to_vec();
    for (ssid, dev) in pairs {
        let mut pair = [&ssid[..], b"/", &dev[..], b", "].concat();
        out_buf.append(&mut pair);
    }
    write_bytes(&mut stdout, out_buf.strip_suffix(b", ").unwrap())?;

    Ok(())
}
