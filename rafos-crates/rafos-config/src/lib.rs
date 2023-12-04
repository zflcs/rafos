//! This crate defines the configurations

#![no_std]
mod kernel;
mod user;

pub use kernel::*;
pub use user::*;