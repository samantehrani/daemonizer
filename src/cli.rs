use clap::{crate_version, App, AppSettings, Arg, ArgMatches};
use std::path::Path;

pub fn parse_arguments() -> ArgMatches<'static> {
    App::new("Daemonizer")
        .about("Daemonizer facilitates mangement of windows services by an abstraction over daemonizing an executable as a service process.")
        .long_about("Daemonizer handles internal windows service control tasks such as communication with service control manager, orchestration, health check, and status information of the service itself. In Microsoft's terminology, Daemonizer will  be categorized as a Service Program.")
        .version(crate_version!())
        .usage("daemonize <name> <executable> [options] [-- <executable args>]")
        .global_settings(&[AppSettings::ColoredHelp])
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .arg(
            Arg::with_name("name")
                .index(1)
                .takes_value(true)
                .required(true)
                .help("Name of the service as registered within SCM datatbase.")
        )
        .arg(
            Arg::with_name("executable")
                .index(2)
                .takes_value(true)
                .required(true)
                .validator(is_file)
                .help("Absolute path to the executable file.")
        )
        .arg(
            Arg::with_name("log_dir")
                .short("l")
                .long("log-dir")
                .takes_value(true)
                .validator(is_dir)
                .help("Absolute path where to start rotating logging for service program.")
        )
        .arg(
            Arg::with_name("executable_args")
                .raw(true)
                .last(true)
                .help("Use `--` separator provide nested arguments to be passed to the daemonized executable.")
        )
        .get_matches()
}

fn is_file(ps: String) -> Result<(), String> {
    let p = Path::new(&ps);
    if p.is_file() {
        if p.is_absolute() {
            Ok(())
        } else {
            Err(String::from("Provided path is not absolute!"))
        }
    } else {
        Err(String::from(
            "Provided path doesn't exist or is not a file!",
        ))
    }
}

fn is_dir(ps: String) -> Result<(), String> {
    let p = Path::new(&ps);
    if p.is_dir() {
        if p.is_absolute() {
            Ok(())
        } else {
            Err(String::from("Provided path is not absolute!"))
        }
    } else {
        Err(String::from(
            "Provided path doesn't exist or is not a directory!",
        ))
    }
}