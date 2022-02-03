use std::ffi::OsString;
use anyhow;
use flexi_logger::{
    colored_opt_format, Cleanup, Criterion, FlexiLoggerError, LogTarget, Logger, LoggerHandle,
    Naming,
};
use std::path::{Path, PathBuf};
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

// create err/result type alias to avoid changes all over the project upon refactors/improvements
pub type Result<T, E = anyhow::Error> = anyhow::Result<T, E>;
pub use anyhow::anyhow as err_from_str;
pub use anyhow::Context as ErrContext;

// rotate logs in directory if specified, otherwise stream to stdout
pub fn initialize_logger(log_dir: Option<&Path>) -> Result<LoggerHandle, FlexiLoggerError> {
    let logger = Logger::with_env_or_str("daemonizer")
        .check_parser_error()
        .unwrap();

    let logger = match log_dir {
        Some(p) => logger
            .log_target(LogTarget::File)
            .directory(PathBuf::from(p))
            .rotate(
                Criterion::Size(1_000_000u64),
                Naming::Timestamps,
                Cleanup::KeepLogFiles(20),
            )
            .cleanup_in_background_thread(true)
            .format_for_files(colored_opt_format),
        _ => logger.log_target(LogTarget::StdOut),
    };
    logger.start()
}

lazy_static! {
    pub static ref IS_DEBUG: Arc<RwLock<Vec<OsString>>> = Arc::new(RwLock::new(vec![]));
}
