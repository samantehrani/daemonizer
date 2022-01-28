use log::*;
use crate::misc::{err_from_str, Result, IS_DEBUG};
use std::sync::{Arc, RwLock};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::mpsc::{channel, Sender},
    thread,
    time::{Duration, SystemTime},
};
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher,
};



pub fn get_args() -> Vec<OsString> {
    match IS_DEBUG.clone().read() {
        Ok(g) => (*g).to_vec(),
        _ => vec![],
    }
}

pub fn push_args(args: Vec<OsString>) {
    match IS_DEBUG.clone().write() {
        Ok(mut g) => (*g).extend(args),
        _ => (),
    };
}

// /// this method is service's main thread. It:
// /// -   checks for update process
// /// -   spawns a new thread to responde to Windows Service Manager events
fn main_function(args: Vec<OsString>) -> Result<(), String> {
    // TODO: make this dynamix this based on env var or input args, or lazy_static mut thread-safe var
    debug!("{:?}", args);
    // debug!("{:?}", executable_path);
    let executable_args = get_args();

    debug!("{:?}", executable_args);

    Ok(())
}

pub fn daemonize(service_name: &str, executable: &Path, arguments: Vec<OsString>) -> Result<()> {
    debug!(
        "Bootstraping Service {:?}. Executable: {:?}. Arguments: {:?}",
        service_name, executable, arguments
    );
    push_args(arguments);
    define_windows_service!(system_service_callback, main_function);
    service_dispatcher::start(service_name, system_service_callback)
        .map_err(|e| err_from_str!("{:?}", e))
}
