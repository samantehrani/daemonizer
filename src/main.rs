use ::daemonizer::{
    cli,
    daemonize,
    install,
    misc::{err_from_str, initialize_logger, Result},
};
use std::{ffi::OsString, path::Path};
use log::*;

fn main() -> Result<()> {
    let cli_matches = cli::parse_arguments();
    let _log_handle = initialize_logger(Some(Path::new("D:\\Users\\saman\\log")))?;
    let result = match cli_matches.subcommand() {
        ("install", Some(m)) => {
            let service_name = m
            .value_of("name")
            .ok_or(err_from_str!("Name must be present!"))?;

            let executable_ps = m
                .value_of("executable")
                .ok_or(err_from_str!("Executable path must be present!"))?;
            let executable_path = Path::new(executable_ps);

            let executable_args: Vec<OsString> = m
                .values_of("executable_args")
                .map_or_else(|| Vec::new(), |vs| vs.collect())
                .iter().map(|v| OsString::from(v))
                .collect();
            install(service_name, executable_path, executable_args)
        },
        ("status", Some(m)) => {
            println!("TODO");
            Ok(())
        },
        ("uninstall", Some(m)) => {
            println!("TODO");
            Ok(())
        },
        ("run", Some(m)) => {
            let service_name = m
            .value_of("name")
            .ok_or(err_from_str!("Name must be present!"))?;

            let executable_ps = m
                .value_of("executable")
                .ok_or(err_from_str!("Executable path must be present!"))?;
            let executable_path = Path::new(executable_ps);

            let executable_args: Vec<OsString> = m
                .values_of("executable_args")
                .map_or_else(|| Vec::new(), |vs| vs.collect())
                .iter().map(|v| OsString::from(v))
                .collect();
            daemonize(service_name, executable_path, executable_args)
        },
        _ => unreachable!(),
    };
    result
}
