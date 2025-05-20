use std::io;

use crate::adapter::Wl;
use crate::api::ScanArgs;
use crate::{Error, write_out};

pub fn scan(mut f: impl io::Write, args: ScanArgs) -> Result<(), Error> {
    const MAX_SIGNAL_STRENGTH: u8 = 100u8;

    let 0u8..=MAX_SIGNAL_STRENGTH = &args.min_strength else {
        return Err(Error::InvalidSignalStrength);
    };

    let process = crate::new();
    let result = process.scan(&args).map_err(Error::CannotScanWiFi)?;
    write_out(&mut f, &result[..])
}
