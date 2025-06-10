use std::env;
use tracing::debug;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry;

pub fn configure_tracing() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let registry = registry();

    if env::var("JSON").is_ok() {
        registry
            .with(
                fmt::layer()
                    .json()
                    .with_file(true)
                    .with_current_span(true)
                    .with_line_number(true)
                    .with_thread_names(true)
                    .with_filter(EnvFilter::from_env("LOG_LEVEL")),
            )
            .init()
    } else {
        registry
            .with(
                fmt::layer()
                    .with_file(false)
                    .with_line_number(false)
                    .with_ansi(true)
                    .without_time()
                    .with_filter(EnvFilter::from_env("LOG_LEVEL")),
            )
            .init();
    };
    debug!("Global tracer configured");
    Ok(())
}
