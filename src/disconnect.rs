use std::{collections::HashMap, error, fmt, io};

use crate::{
    adapter::{self, CARRIAGE_RETURN, LINE_FEED, LOOPBACK_INTERFACE_NAME, Wl},
    write_bytes,
};

#[derive(Debug)]
pub enum Error {
    InvalidActiveSSID(Option<String>),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidActiveSSID(err) => match err {
                Some(err) => write!(f, "unable to get the active SSID: {}", err),
                None => write!(f, "unable to get the active SSID"),
            },
        }
    }
}
impl error::Error for Error {}

pub fn disconnect(ssid: Option<Vec<u8>>, forget: bool) -> Result<(), Box<dyn error::Error>> {
    let ssid = match ssid {
        Some(val) => val,
        None => select_active_ssid()?,
    };

    let process = adapter::new();
    let result = process.disconnect(&ssid, forget)?;

    let mut out_buf = io::stdout();
    write_bytes(&mut out_buf, &result)?;

    Ok(())
}

fn select_active_ssid() -> Result<Vec<u8>, Box<dyn error::Error>> {
    let process = adapter::new();

    let active_ssids = process.get_active_ssids()?;
    let active_ssids_iter = active_ssids.split(|b| b == &LINE_FEED).filter_map(|s| {
        let line = s.strip_suffix(&[CARRIAGE_RETURN]).unwrap_or(s);

        if line.is_empty() || line == LOOPBACK_INTERFACE_NAME {
            None
        } else {
            Some(line)
        }
    });

    let mut ssid_lines = Vec::with_capacity(30);
    let mut ssids = HashMap::new();

    for (idx, ssid) in active_ssids_iter.enumerate() {
        ssid_lines = [
            &ssid_lines[..],
            b"(",
            idx.to_string().as_bytes(),
            b") ",
            ssid,
            b"\n",
        ]
        .concat();
        ssids.insert(idx, ssid);
    }

    let mut stdout = io::stdout();

    let out_buf = &[&ssid_lines[..], b"Select the SSID to disconnect: "].concat();
    write_bytes(&mut stdout, out_buf)?;

    let mut answer_buf = String::new();
    io::stdin()
        .read_line(&mut answer_buf)
        .map_err(|err| Error::InvalidActiveSSID(Some(err.to_string())))?;

    let answer = answer_buf
        .trim()
        .parse::<usize>()
        .map_err(|err| Error::InvalidActiveSSID(Some(err.to_string())))?;

    let ssid = ssids
        .remove(&answer)
        .ok_or(Error::InvalidActiveSSID(None))?;

    Ok(ssid.to_vec())
}
