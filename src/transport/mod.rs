//! Transport base traits for talking to devices

//! Wraps a Stream + Sink backend into a transport
use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use commands::Commands;
use futures::future::BoxFuture;
use std::{pin::Pin, sync::Arc};
use thiserror::Error;
use tokio::sync::{broadcast, Mutex};
use tower::Service;

pub type SharedService = Arc<
    Mutex<
        dyn Service<
                Commands,
                Response = Responses,
                Error = MiniDSPError,
                Future = BoxFuture<'static, Result<Responses, MiniDSPError>>,
            > + Send,
    >,
>;

pub type Transport =
    Pin<Box<dyn StreamSink<'static, Result<Bytes, MiniDSPError>, Bytes, MiniDSPError> + Send>>;

#[cfg(feature = "hid")]
pub mod hid;

#[cfg(feature = "hid")]
use hidapi::HidError;

use crate::{
    commands::{self, Responses},
    utils::StreamSink,
};

pub mod frame_codec;
pub mod multiplexer;
pub use multiplexer::Multiplexer;
pub mod net;

#[derive(Error, Debug)]
pub enum MiniDSPError {
    #[error("An HID error has occurred: {0}")]
    #[cfg(feature = "hid")]
    HIDError(#[from] HidError),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("A malformed packet was received: {0}")]
    MalformedResponse(String),

    #[error("This source was not recognized. Supported types are: 'toslink', 'usb', 'analog'")]
    InvalidSource,

    #[error("There are too many coeffiients in this filter")]
    TooManyCoefficients,

    #[error("Parse error")]
    ParseError(#[from] commands::ParseError),

    #[error("Malformed filter data")]
    MalformedFilterData,

    #[error("Transport error")]
    TransportError(#[from] broadcast::error::RecvError),

    #[error("Transport error: {0}")]
    TransportFailure(String),

    #[error("Transport has closed")]
    TransportClosed,

    #[error("Multiple concurrent commands were sent")]
    ConcurencyError,

    #[error("Internal error")]
    InternalError(#[from] anyhow::Error),
}

#[async_trait]
pub trait Openable {
    async fn open(&self) -> Result<Transport, MiniDSPError>;
}

pub trait IntoTransport {
    fn into_transport(self) -> Transport;
}
