#![cfg(feature = "stream")]
pub mod client;

pub use client::send_message_stream;