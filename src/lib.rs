mod adapter;
pub mod api;
mod connect;
mod disconnect;
mod list_networks;
mod nmcli;
mod scan;
mod status;
mod toggle;

use std::{
    error, fmt,
    io::{self, Write},
};

pub use connect::connect;
pub use disconnect::disconnect;
pub use list_networks::list_networks;
pub use scan::scan;
pub use status::status;
pub use toggle::toggle;

use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    CannotWriteBuffer(io::Error),
    CannotFlushWriter(io::Error),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CannotWriteBuffer(error) => todo!(),
            Error::CannotFlushWriter(error) => todo!(),
        }
    }
}
impl error::Error for Error {}

fn write_bytes(f: &mut impl io::Write, buf: &[u8]) -> Result<(), Error> {
    f.write_all(buf).map_err(Error::CannotWriteBuffer)?;
    f.flush().map_err(Error::CannotFlushWriter)
}
