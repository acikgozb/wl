use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::{Error, adapter::Wl, write_out};

pub fn disconnect(ssid: Option<Vec<u8>>, forget: bool) -> Result<(), Error> {
    let ssid = match ssid {
        Some(val) => val,
        None => select_active_ssid()?,
    };

    let process = crate::new();
    process
        .disconnect(&ssid, forget)
        .map_err(Error::CouldNotDisconnect)
}

fn select_active_ssid() -> Result<Vec<u8>, Error> {
    let process = crate::new();
    let active_ssids = process
        .get_active_ssids()
        .map_err(Error::CannotGetActiveConnections)?;

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
    write_out(io::stdout(), out_buf)?;
    stdout.flush().map_err(Error::CouldNotAskSSID)?;

    let mut answer_buf = String::new();
    io::stdin()
        .read_line(&mut answer_buf)
        .map_err(|err| Error::InvalidActiveSSID(Some(err.to_string())))?;

    let answer = answer_buf
        .trim()
        .parse::<usize>()
        .map_err(|err| Error::InvalidActiveSSID(Some(err.to_string())))?;

    conns.remove(&answer).ok_or(Error::InvalidActiveSSID(None))
}
