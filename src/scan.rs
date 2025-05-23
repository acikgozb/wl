use std::{error, fmt, io};

use crate::adapter::{self, Wl};
use crate::api::ScanArgs;
use crate::write_bytes;

#[derive(Debug)]
pub enum Error {
    InvalidSignalStrength,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // WARN: Implement the missing error messages.
        match self {
            Error::InvalidSignalStrength => todo!(),
        }
    }
}
impl error::Error for Error {}

pub fn scan(f: &mut impl io::Write, args: ScanArgs) -> Result<(), Box<dyn error::Error>> {
    const MAX_SIGNAL_STRENGTH: u8 = 100u8;

    let 0u8..=MAX_SIGNAL_STRENGTH = &args.min_strength else {
        return Err(Error::InvalidSignalStrength)?;
    };

    let process = adapter::new();
    let result = process.scan(&args)?;
    Ok(write_bytes(f, &result[..])?)
}
