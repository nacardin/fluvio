#![feature(trace_macros, generators, specialization)]
#![recursion_limit = "256"]

mod config;
mod core;
mod controllers;
mod metadata;
mod error;
mod init;
mod services;
mod stores;

use init::create_core_services;
use self::error::ScServerError;
use init::start_main_loop;
