#![warn(missing_docs)]
//! This crate provides types that can be reused among multiple different Discord frameworks,
//! without making any assumptions about what type of websocket crate or implementation is being used.

pub mod gateway;
pub mod resources;
