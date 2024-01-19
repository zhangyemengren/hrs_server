//! Hrs System
//!
#![doc = include_str!("../README.md")]

pub mod configuration;
pub mod middlewares;
pub mod routes;
pub mod startup;

pub use middlewares::Jwt;
