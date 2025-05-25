use std::{
    collections::HashMap,
    error, fmt,
    io::{self},
};

use termion::input::TermRead;

use crate::{
    adapter::{self, LINE_FEED, Wl},
    api::ScanArgs,
    write_bytes,
};

/// Defines [`Error`] variants that may return during a connection attempt.
///
/// [`Error`]: `std::error::Error`
#[derive(Debug)]
pub enum Error {
    /// Represents a read failure whilst trying to obtain the SSID password.
    ///
    /// This failure can only come from the underlying stream (e.g. stdin).
    /// It holds the details of the underlying [`io::Error`].
    ///
    /// [`io::Error`]: std::io::Error
    CannotReadPasswd(io::Error),

    /// Represents a read faliure whilst trying to obtain the SSID.
    ///
    /// This error can happen in three ways:
    ///
    /// - From the underlying stdin stream.
    /// - By providing an invalid SSID selection.
    /// - By providing an SSID that does not exist on the given list of SSID's.
    ///
    /// Based on the error case, it holds:
    ///
    /// - [`io::Error`] coming from the stream (stdin).
    /// - The invalid SSID selection.
    /// - None.
    CannotReadSSID(Option<String>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CannotReadPasswd(err) => write!(f, "cannot read passwd from stdin: {}", err),
            Error::CannotReadSSID(err) => match err {
                Some(err) => write!(f, "unable to get the SSID: {}", err),
                None => write!(f, "the given SSID does not exist on the list"),
            },
        }
    }
}
impl error::Error for Error {}

/// Connects to a given WiFi network by using a [`Wl`] implementation.
///
/// If an SSID is not given by the caller, then `connect` shows a list of available networks to choose from.
///
/// If `force_passwd` is set to `true`, the caller is asked to provide a password for the SSID, even if the SSID is a known network.
/// If not, then the password is asked when the provided SSID is not in the known network list of the host.
///
/// The validity of SSID-password pair is delegated to the [`Wl`] implementation. `connect` does not validate the pair.
/// The success result of a connection attempt depends on the [`Wl`] implementation.
///
/// The SSID selection and password are both retrieved from stdin, and the result of the connection attempt is written to stdout.
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function returns [`Error::CannotReadSSID`] if the provided SSID cannot be read, and [`Error::CannotReadPasswd`] if the provided password cannot be read.
///
/// This function can also return an [`NetworkAdapterError`] when the underlying [`Wl`] implementation fails or [`io::Error`] when the successful connection result cannot be written on the stdout stream.
///
/// [`Wl`]: crate::Wl
/// [`Error::CannotReadSSID`]: crate::ConnectError::CannotReadSSID
/// [`Error::CannotReadPasswd`]: crate::ConnectError::CannotReadPasswd
/// [`NetworkAdapterError`]: crate::adapter::Error
/// [`io::Error`]: std::io::Error
pub fn connect(ssid: Option<Vec<u8>>, force_passwd: bool) -> Result<(), Box<dyn error::Error>> {
    let process = adapter::new();

    let ssid = match ssid {
        Some(v) => Ok(v),
        None => ask_ssid(&process),
    }?;

    let is_known_ssid = process.is_known_ssid(&ssid)?;

    let password = match force_passwd {
        true => get_ssid_password(&ssid),
        false => {
            if is_known_ssid {
                Ok(None)
            } else {
                get_ssid_password(&ssid)
            }
        }
    }?;

    let result = process.connect(&ssid, password.as_deref(), is_known_ssid)?;

    let mut out_buf = io::stdout();
    write_bytes(&mut out_buf, &result)?;

    Ok(())
}

fn ask_ssid(process: &impl Wl) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let scan_args = ScanArgs {
        min_strength: 0,
        re_scan: true,
        columns: None,
        get_values: Some(String::from("SSID,SIGNAL")),
    };
    let scan_result = process.scan(&scan_args)?;

    let separator = process.get_field_separator();
    let parsed_scan_result = scan_result
        .split(|b| b == &LINE_FEED)
        .enumerate()
        .filter_map(|(idx, l)| {
            if l.is_empty() {
                None
            } else {
                let fields = l
                    .split(|b| b == &separator)
                    .filter(|b| !b.is_empty())
                    .collect::<Vec<&[u8]>>();

                Some((idx, fields[0], fields[1]))
            }
        });

    let mut ssids = HashMap::new();
    let mut ssid_lines = Vec::new();
    for (idx, ssid, signal) in parsed_scan_result.into_iter() {
        ssids.insert(idx, ssid);

        let line = [
            b"(",
            idx.to_string().as_bytes(),
            b") ",
            ssid,
            b" (sig: ",
            signal,
            b")\n",
        ]
        .concat();
        ssid_lines.push(line);
    }

    let prompt = [
        &ssid_lines.into_iter().flatten().collect::<Vec<u8>>()[..],
        b"Select the SSID to connect: ",
    ]
    .concat();

    write_bytes(&mut io::stdout(), &prompt)?;

    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .map_err(|err| Error::CannotReadSSID(Some(err.to_string())))?;

    let answer = answer
        .trim()
        .parse::<usize>()
        .map_err(|err| Error::CannotReadSSID(Some(err.to_string())))?;

    let ssid = ssids.remove(&answer).ok_or(Error::CannotReadSSID(None))?;

    Ok(ssid.to_vec())
}

fn get_ssid_password(ssid: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn error::Error>> {
    let mut stdin = io::stdin();
    let mut writer = io::stdout();

    let out_buf = [b"Enter the password for ", ssid, b": "].concat();
    write_bytes(&mut writer, &out_buf)?;

    let passwd = stdin
        .read_passwd(&mut writer)
        .map_err(Error::CannotReadPasswd)?;

    Ok(passwd.map(|pw| String::from(pw.trim()).into_bytes()))
}
