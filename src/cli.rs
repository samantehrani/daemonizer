use clap::{crate_version, App, AppSettings, Arg, ArgMatches, crate_authors};
use std::path::Path;

pub fn parse_arguments() -> ArgMatches<'static> {
    App::new(env!("CARGO_CRATE_NAME"))
        .author(crate_authors!())
        .about("Daemonizer facilitates management of daemon applications by creating a unified abstraction over service application on windows, launchd applications on macOS, and launchctl applications in Linux.")
        .version(crate_version!())
        .usage("daemonizer <command> <name> [options]")
        .global_settings(&[AppSettings::ColoredHelp])
        .subcommand(
            App::new("test")
        )
        .subcommand(
            App::new("install")
                .about("Install an executable as a daemon application.")
                .settings(&[AppSettings::ArgRequiredElseHelp])
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .takes_value(true)
                        .required(true)
                        .help("Name of the application to install as a daemon.")
                )
                .arg(
                    Arg::with_name("executable")
                        .short("e")
                        .long("executable")
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
                        .required(false)
                        .raw(true)
                        .last(true)
                        .help("Use `--` separator provide nested arguments to be passed to the daemonized executable.")
                )
        )
        .subcommand(
            App::new("run")
                .about("Install an executable as a daemon application.")
                .settings(&[AppSettings::ArgRequiredElseHelp])
                .arg(
                    Arg::with_name("name")
                        .short("n")
                            .long("name")
                            .takes_value(true)
                            .required(true)
                        .help("Name of the application to install as a daemon.")
                )
                .arg(
                    Arg::with_name("executable")
                        .short("e")
                        .long("executable")
                        .takes_value(true)
                        .required(true)
                        .validator(is_file)
                        .help("Absolute path to the executable file.")
                )
                .arg(
                    Arg::with_name("executable_args")
                        .required(false)
                        .raw(true)
                        .last(true)
                        .help("Use `--` separator provide nested arguments to be passed to the daemonized executable.")
                )
                .about("This command is used internally by Daemonizer in order to spawn the executable as a child process while handling service's lifecycle.")
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
