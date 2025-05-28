use crate::network_interface::NetworkInterfaceHandle;
use ocpp_client::ocpp_1_6::OCPP1_6Error;
use rust_ocpp::v1_6::messages::cancel_reservation::{
    CancelReservationRequest, CancelReservationResponse,
};
use rust_ocpp::v1_6::messages::change_availability::{
    ChangeAvailabilityRequest, ChangeAvailabilityResponse,
};
use rust_ocpp::v1_6::messages::change_configuration::{
    ChangeConfigurationRequest, ChangeConfigurationResponse,
};
use rust_ocpp::v1_6::messages::get_configuration::{
    GetConfigurationRequest, GetConfigurationResponse,
};
use rust_ocpp::v1_6::messages::remote_start_transaction::{
    RemoteStartTransactionRequest, RemoteStartTransactionResponse,
};
use rust_ocpp::v1_6::messages::remote_stop_transaction::{
    RemoteStopTransactionRequest, RemoteStopTransactionResponse,
};
use rust_ocpp::v1_6::messages::reset::{ResetRequest, ResetResponse};
use rust_ocpp::v1_6::messages::trigger_message::{TriggerMessageRequest, TriggerMessageResponse};

#[async_trait::async_trait]
pub trait Ocpp16NetworkInterfaceHandle: NetworkInterfaceHandle {
    async fn send_get_configuration(
        &self,
        request: GetConfigurationRequest,
    ) -> Result<
        Result<GetConfigurationResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    >;

    async fn send_change_configuration(
        &self,
        request: ChangeConfigurationRequest,
    ) -> Result<
        Result<ChangeConfigurationResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    >;

    async fn send_remote_start_transaction(
        &self,
        request: RemoteStartTransactionRequest,
    ) -> Result<
        Result<RemoteStartTransactionResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    >;

    async fn send_remote_stop_transaction(
        &self,
        request: RemoteStopTransactionRequest,
    ) -> Result<
        Result<RemoteStopTransactionResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    >;

    async fn send_trigger_message(
        &self,
        request: TriggerMessageRequest,
    ) -> Result<
        Result<TriggerMessageResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    >;

    async fn send_reset(
        &self,
        request: ResetRequest,
    ) -> Result<
        Result<ResetResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    >;

    async fn send_cancel_reservation(
        &self,
        request: CancelReservationRequest,
    ) -> Result<
        Result<CancelReservationResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    >;

    async fn send_change_availability(
        &self,
        request: ChangeAvailabilityRequest,
    ) -> Result<
        Result<ChangeAvailabilityResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    >;
}
#[cfg(test)]
mockall::mock! {
    pub Ocpp16NetworkInterfaceHandle {}
    #[async_trait::async_trait]
    impl Ocpp16NetworkInterfaceHandle for Ocpp16NetworkInterfaceHandle {
        async fn send_get_configuration(&self, request: GetConfigurationRequest) -> Result<Result<GetConfigurationResponse, OCPP1_6Error>, Box<dyn Error + Send + Sync + 'static>>;
        async fn send_change_configuration(&self, request: ChangeConfigurationRequest) -> Result<Result<ChangeConfigurationResponse, OCPP1_6Error>, Box<dyn Error + Send + Sync + 'static>>;
        async fn send_remote_start_transaction(&self, request: RemoteStartTransactionRequest) -> Result<Result<RemoteStartTransactionResponse, OCPP1_6Error>, Box<dyn Error + Send + Sync + 'static>>;
        async fn send_remote_stop_transaction(&self, request: RemoteStopTransactionRequest) -> Result<Result<RemoteStopTransactionResponse, OCPP1_6Error>, Box<dyn Error + Send + Sync + 'static>>;
        async fn send_trigger_message(&self, request: TriggerMessageRequest) -> Result<Result<TriggerMessageResponse, OCPP1_6Error>, Box<dyn Error + Send + Sync + 'static>>;
        async fn send_reset(&self, request: ResetRequest) -> Result<Result<ResetResponse, OCPP1_6Error>, Box<dyn Error + Send + Sync + 'static>>;
        async fn send_cancel_reservation(&self, request: CancelReservationRequest) -> Result<Result<CancelReservationResponse, OCPP1_6Error>, Box<dyn Error + Send + Sync + 'static>>;
        async fn send_change_availability(&self, request: ChangeAvailabilityRequest) -> Result<Result<ChangeAvailabilityResponse, OCPP1_6Error>, Box<dyn Error + Send + Sync + 'static>>;
    }
    #[async_trait::async_trait]
    impl NetworkInterfaceHandle for Ocpp16NetworkInterfaceHandle {
        async fn disconnect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    }
}
