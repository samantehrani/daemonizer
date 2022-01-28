#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;


pub mod cli;
pub mod misc;

use std::{path::Path, sync::{Arc, RwLock}};
use std::{ffi::OsString};
use misc::{Result};




pub fn daemonize(service_name: &str, executable: &Path, arguments: Vec<OsString>) -> Result<()> {
    Ok(())
}
