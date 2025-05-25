//! A simple wrapper around network backend programs, designed to provide a simple interface for host WiFi management.
//!
//! The WiFi functionality it exposes are executed by the network backends.
//! Here is a list of network backends that are supported:
//!
//! - [`nmcli`] (NetworkManager)
//!
//! To see the interface for each network backend, check out the [`Wl`] trait.
//! To see the available functionality, check out the corresponding functions below:
//!
//! - [`status`]
//! - [`toggle`]
//! - [`list_networks`]
//! - [`scan`]
//! - [`connect`]
//! - [`disconnect`]
//!
//! [`nmcli`]: crate::Nmcli
//! [`Wl`]: crate::Wl
//! [`status`]: crate::status
//! [`toggle`]: crate::toggle
//! [`list_networks`]: crate::list_networks
//! [`scan`]: crate::scan
//! [`connect`]: crate::connect
//! [`disconnect`]: crate::disconnect

mod adapter;
pub mod api;
mod connect;
mod disconnect;
mod list_networks;
mod nmcli;
mod scan;
mod status;
mod toggle;

pub use adapter::{
    CARRIAGE_RETURN, Decimal, Error as NetworkAdapterError, LINE_FEED, LOOPBACK_INTERFACE_NAME, Wl,
};
pub use connect::{Error as ConnectError, connect};
pub use disconnect::{Error as DisconnectError, disconnect};
pub use list_networks::list_networks;
pub use nmcli::Nmcli;
pub use scan::{Error as ScanError, scan};
pub use status::status;
pub use toggle::toggle;

use std::io;

fn write_bytes(f: &mut impl io::Write, buf: &[u8]) -> Result<(), io::Error> {
    f.write_all(buf)?;
    f.flush()
}
