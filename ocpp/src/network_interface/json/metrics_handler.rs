use poem::handler;
use prometheus::{Encoder, TextEncoder};

#[handler]
pub fn metrics_handler() -> String {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();

    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    String::from_utf8(buffer.clone()).unwrap()
}
