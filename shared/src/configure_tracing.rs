use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::trace::Sampler;
use std::env;
use tracing::debug;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry;

pub fn configure_tracing() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let registry = registry();

    let tracer = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(
            opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .build()?,
            opentelemetry_sdk::runtime::Tokio,
        )
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            1.0,
        ))))
        .build();

    let tracer = tracer.tracer("tracing-otel-subscriber");

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
            .with(OpenTelemetryLayer::new(tracer))
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
            .with(OpenTelemetryLayer::new(tracer))
            .init();
    };
    debug!("Global tracer configured");
    Ok(())
}
