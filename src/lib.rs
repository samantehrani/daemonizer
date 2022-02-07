#[cfg_attr(target_os = "macos", path = "macos/mod.rs")]
#[cfg_attr(target_os = "windows", path = "windows/mod.rs")]
mod daemonizer;


pub mod cli;
pub mod misc;

use std::{path::Path};
use std::{ffi::OsString};
use misc::{Result};


pub fn daemonize(service_name: &str, executable: &Path, arguments: Vec<OsString>) -> Result<()> {
    daemonizer::daemonize(service_name, executable, arguments)
}


pub fn install(service_name: &str, executable: &Path, arguments: Vec<OsString>) -> Result<()> {
    daemonizer::install(service_name, executable, arguments)
}
