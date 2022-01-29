use ::daemonizer::{
    cli,
    daemonize,
    misc::{err_from_str, initialize_logger, Result},
};
use std::{ffi::OsString, path::Path};

fn main() -> Result<()> {
    let cli_matches = cli::parse_arguments();
    let _log_handle = initialize_logger(cli_matches.value_of("log_dir").map(|ps| Path::new(ps)))?;

    let result = match cli_matches.subcommand() {
        ("install", Some(m)) => {

        },
        ("status", Some(m)) => {
            println!("TODO");
        },
        ("uninstall", Some(m)) => {
            println!("TODO");
        },
        ("run", Some(m)) => {},
        _ => unreachable!(),
    };

    let service_name = cli_matches
        .value_of("name")
        .ok_or(err_from_str!("Name must be present!"))?;

    let executable_ps = cli_matches
        .value_of("executable")
        .ok_or(err_from_str!("Executable path must be present!"))?;
    let executable_path = Path::new(executable_ps);

    let executable_args: Vec<&str> = cli_matches
        .values_of("executable_args")
        .map_or_else(|| Vec::new(), |vs| vs.collect());

    daemonize(
        service_name,
        executable_path,
        executable_args.iter().map(|&i| OsString::from(i)).collect(),
    )
}
