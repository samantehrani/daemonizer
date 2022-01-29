use log::*;
use std::{
    ffi::OsString,
    path::{Path},
};
use crate::misc::{Result};

pub fn daemonize(service_name: &str, executable: &Path, arguments: Vec<OsString>) -> Result<()> {
    debug!(
        "Bootstraping Service {:?}. Executable: {:?}. Arguments: {:?}",
        service_name, executable, arguments
    );
    Ok(())
}

pub fn install(service_name: &str, executable: &Path, arguments: Vec<OsString>) -> Result<()> {
    debug!(
        "Installing Service {:?}. Executable: {:?}. Arguments: {:?}",
        service_name, executable, arguments
    );
    Ok(())
}
