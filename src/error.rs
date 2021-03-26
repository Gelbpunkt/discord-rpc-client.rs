use serde_json::Error as JsonError;
use std::{
    io::Error as IoError,
    result::Result as StdResult,
    sync::mpsc::{RecvError, RecvTimeoutError as ChannelTimeout, TryRecvError},
};

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    JsonError(JsonError),
    Timeout(ChannelTimeout),
    RecvError,
    Conversion,
    SubscriptionFailed,
    ConnectionClosed,
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::IoError(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::JsonError(err)
    }
}

impl From<ChannelTimeout> for Error {
    fn from(err: ChannelTimeout) -> Self {
        Error::Timeout(err)
    }
}

impl From<RecvError> for Error {
    fn from(_: RecvError) -> Self {
        Error::RecvError
    }
}

impl From<TryRecvError> for Error {
    fn from(_: TryRecvError) -> Self {
        Error::RecvError
    }
}

pub type Result<T> = StdResult<T, Error>;
