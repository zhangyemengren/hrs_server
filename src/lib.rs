//! Hrs System
//!
#![doc = include_str!("../README.md")]

pub mod configuration;
pub mod middlewares;
pub mod response;
pub mod routes;
pub mod startup;
pub mod validator;

pub use middlewares::*;
pub use validator::*;
