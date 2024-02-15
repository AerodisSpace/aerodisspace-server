pub use tracing;

pub mod logger {
    use tracing::{info, Level};
    use tracing_appender::rolling;

    pub fn log_init(write: bool, level: &str) {
        let level = match level.to_lowercase().as_str() {
            "trace" => Level::TRACE,
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => Level::TRACE,
        };

        // log files
        if write {
            let path_log_service = format!("./logs/{}", "aerodis");
            let file = rolling::minutely(path_log_service, "debug.log");
            tracing_subscriber::fmt()
                .pretty()
                .with_thread_names(true)
                .with_writer(file)
                .with_ansi(false)
                .with_max_level(level)
                .init();
        } else {
            // stdout
            tracing_subscriber::fmt()
                .pretty()
                .with_thread_names(true)
                .with_ansi(true)
                .with_max_level(level)
                .init();
        }

        info!("Logger initialized");
    }
}
