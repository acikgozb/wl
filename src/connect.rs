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

#[derive(Debug)]
pub enum Error {
    CannotReadPasswd(io::Error),
    CannotReadSSID(Option<String>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // WARN: Implement the missing error messages.
        match self {
            Error::CannotReadPasswd(error) => todo!(),
            Error::CannotReadSSID(error) => todo!(),
        }
    }
}
impl error::Error for Error {}

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
