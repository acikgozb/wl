use std::{collections::HashMap, error, fmt, io};

use crate::{
    adapter::{self, Wl},
    write_bytes,
};

#[derive(Debug)]
pub enum Error {
    InvalidActiveSSID(Option<String>),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // WARN: Implement the missing error messages.
        match self {
            Error::InvalidActiveSSID(_) => todo!(),
        }
    }
}
impl error::Error for Error {}

pub fn disconnect(ssid: Option<Vec<u8>>, forget: bool) -> Result<(), Box<dyn error::Error>> {
    let ssid = match ssid {
        Some(val) => val,
        None => select_active_ssid()?,
    };

    let process = crate::new();
    let result = process.disconnect(&ssid, forget)?;

    let mut out_buf = io::stdout();
    write_bytes(&mut out_buf, &result)?;

    Ok(())
}

fn select_active_ssid() -> Result<Vec<u8>, Box<dyn error::Error>> {
    let process = crate::new();
    let active_ssids = process.get_active_ssids()?;

    let mut ssids = Vec::new();
    let mut conns = HashMap::new();

    for (idx, ssid) in active_ssids.into_iter().enumerate() {
        ssids = [
            &ssids[..],
            b"(",
            idx.to_string().as_bytes(),
            b") ",
            &ssid[..],
            b"\n",
        ]
        .concat();
        conns.insert(idx, ssid);
    }

    let mut stdout = io::stdout();

    let out_buf = &[
        b"Select the SSID you want to disconnect from:\n",
        &ssids[..],
        b"\n> ",
    ]
    .concat();
    write_bytes(&mut stdout, out_buf)?;

    let mut answer_buf = String::new();
    io::stdin()
        .read_line(&mut answer_buf)
        .map_err(|err| Error::InvalidActiveSSID(Some(err.to_string())))?;

    let answer = answer_buf
        .trim()
        .parse::<usize>()
        .map_err(|err| Error::InvalidActiveSSID(Some(err.to_string())))?;

    let ssid = conns
        .remove(&answer)
        .ok_or(Error::InvalidActiveSSID(None))?;

    Ok(ssid)
}
