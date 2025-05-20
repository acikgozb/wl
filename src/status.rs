use crate::{Error, adapter::Wl, write_out};

pub fn status() -> Result<(), Error> {
    let process = crate::new();
    let pairs = process
        .get_active_ssid_dev_pairs()
        .map_err(Error::CannotGetActiveConnections)?;

    let wifi_status = process
        .get_wifi_status()
        .map_err(Error::CannotGetWifiStatus)?;

    let out_buf = format!("wifi: {}\n", wifi_status);
    write_out(out_buf.as_bytes())?;

    let mut out_buf: Vec<u8> = b"connected networks: ".to_vec();
    for (ssid, dev) in pairs {
        let mut pair = [&ssid[..], b"/", &dev[..], b", "].concat();
        out_buf.append(&mut pair);
    }
    write_out(out_buf.strip_suffix(b", ").unwrap())?;

    Ok(())
}
