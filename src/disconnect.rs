use std::{
    collections::HashMap,
    error, fmt,
    io::{self},
};

use crate::{
    adapter::{self, CARRIAGE_RETURN, LINE_FEED, LOOPBACK_INTERFACE_NAME, Wl},
    write_bytes,
};

/// Defines [`Error`] variants that may return during a disconnect process.
///
/// [`Error`]: std::error::Error
#[derive(Debug)]
pub enum Error {
    /// Represents a read faliure whilst trying to obtain the SSID.
    ///
    /// This read failure can happen in three ways:
    ///
    /// 1 - From the underlying stream (stdin).
    /// 2 - By providing an input in the wrong format.
    /// 3 - By providing an SSID that does not exist on the given list of SSID's.
    /// Based on the error case, it holds:
    ///
    /// - [`io::Error`] coming from the stream (stdin).
    /// - The invalid SSID format.
    /// - None.
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

/// Disconnects from a WiFi network by using a [`Wl`] implementation.
///
/// If an SSID is not given by the caller, then `disconnect` shows a list of active networks to choose from.
///
/// If `forget` is set to `true`, then the selected SSID is disconnected and removed from the known network list.
/// If `forget` is set to `false`, then the selected SSID is only disconnected.
/// The successful disconnection result format depends on the [`Wl`] implementation.
///
/// The SSID selection is retrieved from stdin, and the result of the disconnect is written to stdout.
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function returns [`Error::InvalidActiveSSID`] if the provided SSID cannot be read.
///
/// This function can also return an [`NetworkAdapterError`] when the underlying [`Wl`] implementation fails or [`io::Error`] when the successful disconnection result cannot be written on the stdout stream.
///
/// [`Wl`]: crate::Wl
/// [`Error::InvalidActiveSSID`]: crate::DisconnectError::InvalidActiveSSID
/// [`NetworkAdapterError`]: crate::adapter::Error
/// [`io::Error`]: std::io::Error
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
