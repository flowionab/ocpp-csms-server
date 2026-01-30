mod handle_new_request;
mod handle_new_websocket_connection;
mod handle_websocket_message;
mod ocpp1_6;
mod ocpp2_0_1;

pub const OCPP_CALL: i64 = 2;
pub const OCPP_CALL_RESULT: i64 = 3;
pub const OCPP_ERROR: i64 = 4;

lazy_static! {
    pub static ref OCPP_CALLS: HistogramVec = register_histogram_vec!(
        "ocpp_csms_server_charger_to_server_calls",
        "Histogram of calls made from the charger towards the server",
        &["protocol", "action"]
    )
    .unwrap();
}

pub use self::handle_new_request::handle_new_request;
use lazy_static::lazy_static;
use prometheus::{HistogramVec, register_histogram_vec};
