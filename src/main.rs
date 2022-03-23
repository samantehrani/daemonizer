use ::daemonizer::{
    cli,
    daemonize,
    daemonizer,
    install,
    misc::{err_from_str, initialize_logger, Result},
};
use std::{ffi::OsString, path::Path};

fn main() -> Result<()> {
    let cli_matches = cli::parse_arguments();
    let _log_handle;
    let result = match cli_matches.subcommand() {
        ("install", Some(m)) => {
            _log_handle = initialize_logger(None)?;
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
        ("status", Some(_m)) => {
            println!("TODO");
            Ok(())
        },
        ("uninstall", Some(_m)) => {
            println!("TODO");
            Ok(())
        },
        ("run", Some(m)) => {
            _log_handle = initialize_logger(Some(Path::new("C:\\Users\\saman\\Desktop")))?;
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
        ("test", Some(_)) => {
            // daemonizer::test()
            println!("test placeholder");
            Ok(())
        },
        _ => unreachable!(),
    };
    result
}
