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
