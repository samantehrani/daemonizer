use log::*;
use crate::misc::{err_from_str, Result, IS_DEBUG};
use std::{
    process::Command,
    ffi::OsString,
    path::{Path},
    sync::mpsc::{channel, Sender},
    time::{Duration, SystemTime},
};
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType, ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher,
    service_manager::{ServiceManager, ServiceManagerAccess},
};
use win32job::{Job};

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

/// send appropriate actions back to main thread to be handled.
/// see return values and remarks at https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nc-winsvc-lphandler_function_ex?redirectedfrom=MSDN
fn event_handler_cb(
    event: ServiceControl,
    tx: &Sender<ServiceControl>,
) -> ServiceControlHandlerResult {
    match event {
        ServiceControl::Stop => tx
            .send(event)
            .map(|_| ServiceControlHandlerResult::NoError)
            .unwrap_or_else(|_| {
                error!("failed to send stop event to main thread!");
                ServiceControlHandlerResult::NoError
            }),
        ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
        _ => ServiceControlHandlerResult::NotImplemented,
    };
    ServiceControlHandlerResult::NoError
}


// /// this method is service's main thread. It:
// /// -   checks for update process
// /// -   spawns a new th  read to responde to Windows Service Manager events
fn main_function(args: Vec<OsString>) -> Result<(), Box<dyn std::error::Error>> {
    let name = args.get(0);
    debug!("in service main");
    // TODO: figure out why this doesn't work
    // debug!("{:?}", args);
    // work around
    let daemonizer_args = get_args();

    debug!("{:?}", daemonizer_args);
    // comm. channel for ServiceEventHandler thread <-> main thread
    let (ctrl_tx, ctrl_rx) = channel::<ServiceControl>();

    // register event handlers and set running status
    let r_handle = service_control_handler::register(
        name.ok_or_else(|| {
            error!("name not provided uin service entry arguments");
            "name not provided uin service entry arguments"
        })?,
        move |e| event_handler_cb(e, &ctrl_tx),
    )
    .map_err(|e| {
        error!("{:?}", e);
        "failed to create handle"
    })?;

    // create job group, assign current process and set children to terminate upon termination.
    // https://docs.microsoft.com/en-us/windows/win32/procthread/job-objects

    let job = Job::create()?;
    let mut info = job.query_extended_limit_info()?;

    info.limit_kill_on_job_close();

    job.set_extended_limit_info(&mut info)?;
    job.assign_current_process()?;

    // run executable and get the process id
    let mut p = Command::new("C:\\Program Files\\Mozilla Firefox\\firefox.exe");

    use std::os::windows::process::CommandExt;

    p.creation_flags(winapi::um::winbase::CREATE_NEW_CONSOLE | winapi::um::winbase::DETACHED_PROCESS);
    let result = p.spawn();

    let child;
    if result.is_err() {
        debug!("{:?}", result.err());
        r_handle
        .set_service_status(ServiceStatus {
            service_type: ServiceType::OWN_PROCESS,
            current_state: ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0, // must be zero since service does not implement pending start, stop, pause.
            wait_hint: std::time::Duration::from_secs(0),
            process_id: Some(std::process::id()),
        })
        .map_err(|e| {
            error!("{:?}", e);
            "failed to set service status stopped"
        })?;
        return Ok(());
    } else {
        child = result?;
    }

    r_handle
        .set_service_status(ServiceStatus {
            service_type: ServiceType::OWN_PROCESS,
            current_state: ServiceState::Running,
            controls_accepted: ServiceControlAccept::STOP,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0, // must be zero since service does not implement pending start, stop, pause.
            wait_hint: std::time::Duration::from_secs(0),
            process_id: Some(std::process::id()),
        })
        .map_err(|e| {
            error!("{:?}", e);
            "failed to set service status"
        })?;
    let mut last_update_time = SystemTime::UNIX_EPOCH;
    loop {
        match ctrl_rx.recv_timeout(Duration::from_millis(200)) {
            Ok(e) => match e {
                ServiceControl::Stop => break,
                _ => unimplemented!(),
            },
            // on timeout check for package updates
            Err(_) => match last_update_time.elapsed() {
                Ok(d) if d > Duration::from_secs(2) => {
                    last_update_time = SystemTime::now();
                    debug!("{:?}", child.id());
                }
                Ok(_) => continue,
                Err(e) => {
                    error!("system time error: {}", e);
                    continue;
                }
            },
        }
    }
    info!("stopping service!");

    // clean up the service, set stop status!
    r_handle
        .set_service_status(ServiceStatus {
            service_type: ServiceType::OWN_PROCESS,
            current_state: ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0, // must be zero since service does not implement pending start, stop, pause.
            wait_hint: std::time::Duration::from_secs(0),
            process_id: Some(std::process::id()),
        })
        .map_err(|e| {
            error!("{:?}", e);
            "failed to set service status stopped"
        })?;

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

pub fn install(service_name: &str, executable: &Path, arguments: Vec<OsString>) -> Result<()> {
    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    //TODO: option to manage non-local stations ?
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let daemonizer_binary_path = ::std::env::current_exe().unwrap();
    println!("{:?}", daemonizer_binary_path);
    let mut args = vec![
        OsString::from("run"),
        OsString::from("-n"), OsString::from(service_name),
        OsString::from("-e"), OsString::from(executable.as_os_str()),
        OsString::from("--"),
    ];
    args.extend(arguments.iter().map(|v| v.clone()));
    println!("{:?}", args);
    let service_info = ServiceInfo {
        name: OsString::from(service_name),
        display_name: OsString::from(service_name),
        service_type: ServiceType::OWN_PROCESS, // TODO: other types ?
        start_type: ServiceStartType::AutoStart, // TODO: optional start type
        error_control: ServiceErrorControl::Normal, // TODO: optional error control input
        executable_path: daemonizer_binary_path,
        launch_arguments: args,
        dependencies: vec![], // TODO: check out what this does
        account_name: None, // TODO: need these in run as user mode
        account_password: None,
    };
    let _service = service_manager.create_service(&service_info, ServiceAccess::CHANGE_CONFIG)?;

    Ok(())
}
