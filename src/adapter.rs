use std::{error, fmt, io};

use crate::{api::ScanArgs, nmcli};

/// Represents the line feed byte that can be used to split
/// a byte slice into lines.
///
/// Some programs represent lines by appending both the line feed
/// and the carriage return bytes.
/// To ensure that the byte slice is splitted into lines, use this
/// along with [`CARRIAGE_RETURN`].
///
/// [`CARRIAGE_RETURN`] - crate::adapter::CARRIAGE_RETURN
pub const LINE_FEED: u8 = 0xA;

/// Represents the carriage return byte that can be used to split
/// a byte slice into lines.
///
/// Some programs represent lines by appending both the line feed
/// and the carriage return bytes.
/// To ensure that the byte slice is splitted into lines, use this
/// along with [`LINE_FEED`].
///
/// [`LINE_FEED`] - crate::adapter::LINE_FEED
pub const CARRIAGE_RETURN: u8 = 0xD;

/// Represents the loopback interface name.
///
/// Some network backends contain the loopback interface in their output.
/// If needed, this can be used to filter out the interface from the output.
pub const LOOPBACK_INTERFACE_NAME: &[u8] = b"lo";

/// The main interface that should be implemented by all network backends.
///
/// Methods of this trait represent the core functionality that should be provided by each network backend.
///
/// The implementors of `Wl` may or may not encode their Ok result - it depends on the functionality of each individual method.
///
/// The callers of `Wl` should not assume anything about the return format other than being a byte stream. The format may differ for each method, and the implementors should document them wherever possible.
///
/// If a method returns a terse output for scripting purposes, then the implementor should mention it.
///
/// To see the available Error's, check out [`Error`].
///
/// [`Error`]: crate::adapter::Error
pub trait Wl {
    /// Provides the byte that is used for terse outputs.
    ///
    /// The byte that is returned from this method should be the one that is used in terse outputs that are returned from other methods to ensure that caller can rely on this method.
    fn get_field_separator(&self) -> u8;

    /// Provides the WiFi status.
    fn get_wifi_status(&self) -> Result<Vec<u8>, Error>;

    /// Toggles the WiFi status.
    fn toggle_wifi(&self) -> Result<Vec<u8>, Error>;

    /// Lists the known networks on the host.
    fn list_networks(&self, show_active: bool, show_ssid: bool) -> Result<Vec<u8>, Error>;

    /// Provides a stream of SSID-Device pairs.
    fn get_active_ssid_dev_pairs(&self) -> Result<Vec<u8>, Error>;

    /// Provides a stream of active SSID's - aka. connected networks.
    fn get_active_ssids(&self) -> Result<Vec<u8>, Error>;

    /// Disconnects the host from the given SSID.
    ///
    /// If `forget` is set, then this method removes the given SSID from the known network list of the host.
    fn disconnect(&self, ssid: &[u8], forget: bool) -> Result<Vec<u8>, Error>;

    /// Provides a stream of networks that can be connected.
    ///
    /// The implementors of this method should be able to provide both
    /// human-readable and terse output.
    ///
    /// The values of [`ScanArgs`] may differ between each network backend.
    /// The implementors should document the available options for callers.
    ///
    /// [`ScanArgs`]: crate::api::ScanArgs
    fn scan(&self, args: &ScanArgs) -> Result<Vec<u8>, Error>;

    /// Provides whether the given SSID exists under the known network list
    /// of the host or not.
    fn is_known_ssid(&self, ssid: &[u8]) -> Result<bool, Error>;

    /// Connects the host to the given SSID.
    ///
    /// The implementors should validate whether the given SSID-password
    /// pair is valid or not.
    /// The callers are responsible from providing the SSID-password pair to the implementors.
    fn connect(
        &self,
        ssid: &[u8],
        passwd: Option<&[u8]>,
        is_known_ssid: bool,
    ) -> Result<Vec<u8>, Error>;
}

/// Initializes a new network backend adapter to the caller.
///
/// If the network backend relies on an external program, this
/// function does not validate the existence of that external program.
pub fn new() -> impl Wl {
    nmcli::Nmcli::new()
}

/// The main Error that is returned from the implementors of `Wl`.
///
/// The variants have 3 things:
/// - A high level context about the error.
/// - A detailed error about the underlying functionality.
/// - An exit code that can be used to terminate the caller.
#[derive(Debug)]
pub enum Error {
    CannotGetWiFiStatus((io::Error, i32)),
    CannotToggleWiFi((io::Error, i32)),
    CannotListNetworks((io::Error, i32)),
    CannotGetActiveConnections((io::Error, i32)),
    CannotGetSSIDStatus((io::Error, i32)),
    CannotDisconnect((io::Error, i32)),
    CannotScanWiFi((io::Error, i32)),
    CannotConnect((io::Error, i32)),
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CannotGetWiFiStatus((err, _)) => {
                write!(f, "unable to get the WiFi status: {}", err)
            }
            Error::CannotToggleWiFi((err, _)) => write!(f, "unable to toggle WiFi: {}", err),
            Error::CannotListNetworks((err, _)) => {
                write!(f, "unable to list the networks: {}", err)
            }
            Error::CannotGetActiveConnections((err, _)) => {
                write!(f, "unable to get the active connections: {}", err)
            }
            Error::CannotGetSSIDStatus((err, _)) => {
                write!(f, "unable to get the SSID status: {}", err)
            }
            Error::CannotDisconnect((err, _)) => write!(f, "unable to disconnect: {}", err),
            Error::CannotScanWiFi((err, _)) => {
                write!(f, "unable to scan the available networks: {}", err)
            }
            Error::CannotConnect((err, _)) => {
                write!(f, "unable to connect to the network: {}", err)
            }
        }
    }
}

/// A wrapper type of u8.
///
///
/// It is designed to convert the given byte into its decimal representation.
/// Implements [`From<&[u8]>`].
///
/// # Example
///
/// ```
/// use wl::Decimal;
///
/// let dec = 5;
/// let byte = b"5".as_slice();
///
/// assert_eq!(dec, Decimal::from(byte).inner())
/// ```
///
/// [`From<&[u8]>`]: std::convert::From
pub struct Decimal(u8);

impl From<&[u8]> for Decimal {
    fn from(value: &[u8]) -> Self {
        Self(value.iter().fold(0, |acc, b| acc * 10 + (b - b'0')))
    }
}

impl Decimal {
    /// Provides the underlying decimal value to the caller.
    pub fn inner(&self) -> u8 {
        self.0
    }
}
