use std::{error, fmt, io};

use crate::adapter::{self, Wl};
use crate::api::ScanArgs;
use crate::write_bytes;

/// Defines [`Error`] variants that may return during a scan.
///
/// [`Error`]: std::error::Error
#[derive(Debug)]
pub enum Error {
    /// Represents an invalid signal strength that cannot be used
    /// to filter the scan list.
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

/// Writes the list of the available WiFi networks. To see a list of the known WiFi networks, please refer to [`list_networks`] instead.
///
/// The list is retrieved by using a [`Wl`] implementation.
/// The list is written to the provided [`io::Write`] implementation.
///
/// The default list format depends on the [`Wl`] implementation.
/// [`ScanArgs`] is used to manipulate the list.
///
/// To see how [`ScanArgs`] manipulates the list, check out the network backend modules:
///
/// - nmcli: [`nmcli::scan`]
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function returns [`Error::InvalidSignalStrength`] if the provided signal strength is above 100.
///
/// This function can also return an [`NetworkAdapterError`] when the underlying [`Wl`] implementation fails or [`io::Error`] when the information cannot be written on the given [`io::Write`].
///
/// [`Error::InvalidSignalStrength`]: crate::scan::Error::InvalidSignalStrength
/// [`nmcli::scan`]: crate::Nmcli::scan
/// [`NetworkAdapterError`]: crate::adapter::Error
/// [`io::Error`]: std::io::Error
/// [`list_networks`]: crate::list_networks
pub fn scan(f: &mut impl io::Write, args: ScanArgs) -> Result<(), Box<dyn error::Error>> {
    const MAX_SIGNAL_STRENGTH: u8 = 100u8;

    let 0u8..=MAX_SIGNAL_STRENGTH = &args.min_strength else {
        return Err(Error::InvalidSignalStrength(args.min_strength))?;
    };

    let process = adapter::new();
    let result = process.scan(&args)?;
    Ok(write_bytes(f, &result[..])?)
}
