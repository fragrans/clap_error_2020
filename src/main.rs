use slog::Drain;
use slog::{crit, debug, error, info, trace, warn};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = clap::load_yaml!("my.yml");
    let mut app = clap::App::from(yaml);
    app = app.version(clap::crate_version!());
    let mut help_s = Vec::new();
    app.write_long_help(&mut help_s)
        .map_err(|e| anyhow::anyhow!("e: {}", e))?;
    let mut version_s = Vec::new();
    app.write_long_version(&mut version_s)
        .map_err(|e| anyhow::anyhow!("e: {}", e))?;

    // app will be consumed.
    let master_matches = app.get_matches();

    // logging facility
    // Critical
    // Error
    // Warning
    // Info
    // Debug
    // Trace
    let min_log_level = match master_matches.occurrences_of("verbose") {
        0 => slog::Level::Error,
        1 => slog::Level::Warning,
        2 => slog::Level::Info,
        3 => slog::Level::Debug,
        4 | _ => slog::Level::Trace,
    };
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let drain = slog_term::FullFormat::new(plain)
        .build()
        .filter_level(min_log_level)
        .fuse();
    // let drain = slog_async::Async::new(drain).build().fuse();

    let logger = slog::Logger::root(drain, slog::o!());

    let _guard = slog_scope::set_global_logger(logger);

    info!(slog_scope::logger(), "Logging filter severity: {:#?}", min_log_level);

    trace!(slog_scope::logger(), "trace!");
    debug!(slog_scope::logger(), "debug!");
    info!(slog_scope::logger(), "info!");
    warn!(slog_scope::logger(), "warn!");
    error!(slog_scope::logger(), "error!");
    crit!(slog_scope::logger(), "crit!");

    trace!(slog_scope::logger(), "mathes: {:#?}", master_matches);
    println!("mathes: {:#?}", master_matches);
    Ok(())
}
