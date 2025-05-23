use std::{error, fmt, io};

use crate::adapter::{self, Wl};
use crate::api::ScanArgs;
use crate::write_bytes;

#[derive(Debug)]
pub enum Error {
    InvalidSignalStrength(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidSignalStrength(s) => write!(
                f,
                "the given signal strength {} is not in limits (1..100)",
                s
            ),
        }
    }
}
impl error::Error for Error {}

pub fn scan(f: &mut impl io::Write, args: ScanArgs) -> Result<(), Box<dyn error::Error>> {
    const MAX_SIGNAL_STRENGTH: u8 = 100u8;

    let 0u8..=MAX_SIGNAL_STRENGTH = &args.min_strength else {
        return Err(Error::InvalidSignalStrength(args.min_strength))?;
    };

    let process = adapter::new();
    let result = process.scan(&args)?;
    Ok(write_bytes(f, &result[..])?)
}
