//! `cowbird` is a tool for triggering endpoint detection and response (EDR)
//! agents, and produces structured logs for regression testing.
//! Please see [github](https://github.com/erikwilson/cowbird) for command-line
//! usage.

#![allow(unused)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate slog;

pub mod file;
pub mod network;
pub mod process;
pub mod util;

pub mod cli;
pub mod cmd;
pub mod log;
