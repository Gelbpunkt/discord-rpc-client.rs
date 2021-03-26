#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate byteorder;
extern crate bytes;
#[cfg(windows)]
extern crate named_pipe;
extern crate uuid;

#[macro_use]
mod macros;
pub mod client;
mod connection;
mod error;
pub mod models;
mod utils;

pub use client::Client;
pub use connection::{Connection, SocketConnection};
