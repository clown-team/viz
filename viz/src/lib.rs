#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

mod server;

pub use viz_core::*;
pub use viz_router::*;

// https://github.com/hyperium/hyper/commit/ce72f73464d96fd67b59ceff08fd424733b43ffa#diff-1eaa7c1646ca4a8c2741ab2b4f80d22ab646d8ab031f99925a3adcc3ac242dcd
pub use hyper::server::accept::from_stream as accept_from_stream;
pub use hyper::Server;
pub use server::{ServiceMaker, Stream};

#[cfg(feature = "handlers")]
pub use viz_handlers as handlers;
#[cfg(feature = "macros")]
pub use viz_macros::handler;

#[cfg(any(feature = "rustls", feature = "native-tls"))]
pub mod tls {
    pub use crate::server::tls::*;
}
