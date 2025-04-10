mod ocpp_api_client;
pub mod types;

pub mod ocpp_csms_server {
    tonic::include_proto!("ocpp_csms_server");
}

pub use self::ocpp_api_client::OcppApiClient;
