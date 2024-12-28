use log::{debug, error, info, trace, warn};
use std::env;
use std::fs;

pub fn init() -> Result<(), fern::InitError> {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_string());
    let log_level = log_level
        .parse::<log::LevelFilter>()
        .unwrap_or(log::LevelFilter::Info);

    let mut builder = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stderr());

    if let Ok(log_file) = env::var("LOG_FILE") {
        let log_file = fs::File::create(log_file)?;
        builder = builder.chain(log_file);
    }

    builder.apply()?;

    info!("Logging initialized");
    trace!("This is a trace message");
    debug!("This is a debug message");
    warn!("This is a warning message");
    error!("This is an error message");

    Ok(())
}
