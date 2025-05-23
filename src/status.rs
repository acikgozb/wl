use std::{error, io};

use crate::{
    adapter::{self, CARRIAGE_RETURN, LINE_FEED},
    write_bytes,
};

pub fn status() -> Result<(), Box<dyn error::Error>> {
    let mut stdout = io::stdout();
    let process = adapter::new();

    write_wifi_status(&mut stdout, &process)?;
    write_active_ssid_dev_pairs(&mut stdout, &process)?;
    Ok(())
}

fn write_wifi_status(
    f: &mut impl io::Write,
    process: &impl adapter::Wl,
) -> Result<(), Box<dyn error::Error>> {
    let wifi_status = process.get_wifi_status()?;

    let wifi_status = [b"wifi: ", &wifi_status[..], b" \n"].concat();
    write_bytes(f, &wifi_status)?;
    Ok(())
}

fn write_active_ssid_dev_pairs(
    f: &mut impl io::Write,
    process: &impl adapter::Wl,
) -> Result<(), Box<dyn error::Error>> {
    let pairs = process.get_active_ssid_dev_pairs()?;
    let field_separator = process.get_field_separator();

    let pair_iter = pairs.split(|b| b == &LINE_FEED).filter_map(|s| {
        let line = s.strip_suffix(&[CARRIAGE_RETURN]).unwrap_or(s);

        if line.is_empty() {
            None
        } else {
            let pair = line
                .split(|b| b == &field_separator)
                .collect::<Vec<&[u8]>>();

            Some((pair[0].to_vec(), pair[1].to_vec()))
        }
    });

    let mut active_ssid_dev_pairs = ["connected networks: ".as_bytes()].concat();
    for (ssid, dev) in pair_iter {
        let mut pair = [&ssid[..], b"/", &dev[..], b", "].concat();
        active_ssid_dev_pairs.append(&mut pair);
    }
    write_bytes(f, active_ssid_dev_pairs.strip_suffix(b", ").unwrap())?;

    Ok(())
}
