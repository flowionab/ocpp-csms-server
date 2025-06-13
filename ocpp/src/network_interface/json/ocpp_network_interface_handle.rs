use crate::network_interface::network_interface_handle::NetworkInterfaceHandle;
use crate::network_interface::{
    Ocpp16NetworkInterfaceHandle, Ocpp2_0_1NetworkInterfaceHandle, OcppProtocol,
};
use futures::stream::SplitSink;
use futures::SinkExt;
use ocpp_client::ocpp_1_6::OCPP1_6Error;
use ocpp_client::ocpp_2_0_1::OCPP2_0_1Error;
use poem::web::websocket::Message::Text;
use poem::web::websocket::{Message, WebSocketStream};
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
use rust_ocpp::v1_6::messages::reserve_now::{ReserveNowRequest, ReserveNowResponse};
use rust_ocpp::v1_6::messages::reset::{ResetRequest, ResetResponse};
use rust_ocpp::v1_6::messages::trigger_message::{TriggerMessageRequest, TriggerMessageResponse};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use shared::Config;
use std::collections::BTreeMap;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot::Sender;
use tokio::sync::{oneshot, MappedMutexGuard, Mutex, MutexGuard};
use tokio::time::timeout;
use tracing::{info, warn};
use uuid::Uuid;

pub struct OcppNetworkInterfaceHandle {
    charger_id: String,
    config: Arc<Config>,
    sink: Mutex<Option<SplitSink<WebSocketStream, Message>>>,
    ocpp_1_6_message_queue: Mutex<BTreeMap<String, Sender<Result<Value, OCPP1_6Error>>>>,
    ocpp_2_0_1_message_queue: Mutex<BTreeMap<String, Sender<Result<Value, OCPP2_0_1Error>>>>,
}

impl OcppNetworkInterfaceHandle {
    pub fn new(config: Arc<Config>, charger_id: &str) -> Self {
        Self {
            charger_id: charger_id.to_string(),
            config,
            sink: Mutex::new(None),
            ocpp_1_6_message_queue: Mutex::new(BTreeMap::new()),
            ocpp_2_0_1_message_queue: Mutex::new(BTreeMap::new()),
        }
    }

    pub async fn attach_sink(&self, sink: SplitSink<WebSocketStream, Message>) {
        let mut lock = self.sink.lock().await;
        *lock = Some(sink);
    }

    async fn use_sink(
        &self,
    ) -> Result<MappedMutexGuard<SplitSink<WebSocketStream, Message>>, Box<dyn Error + Send + Sync>>
    {
        let lock = self.sink.lock().await;

        match MutexGuard::try_map(lock, |i| i.as_mut()) {
            Ok(guard) => Ok(guard),
            Err(_) => Err("Sink is not attached".into()),
        }
    }

    pub async fn send_raw(&self, payload: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut sink = self.use_sink().await?;
        sink.send(Message::Text(payload)).await?;
        Ok(())
    }

    pub async fn response_ocpp_1_6(&self, message_id: &str, result: Result<Value, OCPP1_6Error>) {
        let mut lock = self.ocpp_1_6_message_queue.lock().await;
        match result {
            Ok(value) => {
                if let Some(sender) = lock.remove(message_id) {
                    if sender.send(Ok(value)).is_err() {
                        warn!("The message had timed out");
                    }
                } else {
                    warn!("We were not expecting this message, dropping it...")
                }
            }
            Err(err) => {
                if let Some(sender) = lock.remove(message_id) {
                    if sender.send(Err(err)).is_err() {
                        warn!("The message had timed out");
                    }
                } else {
                    warn!("We were not expecting this message, dropping it...")
                }
            }
        }
    }

    pub async fn response_ocpp_2_0_1(
        &self,
        message_id: &str,
        result: Result<Value, OCPP2_0_1Error>,
    ) {
        let mut lock = self.ocpp_2_0_1_message_queue.lock().await;
        match result {
            Ok(value) => {
                if let Some(sender) = lock.remove(message_id) {
                    if sender.send(Ok(value)).is_err() {
                        warn!("The message had timed out");
                    }
                } else {
                    warn!("We were not expecting this message, dropping it...")
                }
            }
            Err(err) => {
                if let Some(sender) = lock.remove(message_id) {
                    if sender.send(Err(err)).is_err() {
                        warn!("The message had timed out");
                    }
                } else {
                    warn!("We were not expecting this message, dropping it...")
                }
            }
        }
    }

    async fn send_ocpp_1_6<T: Serialize, R: DeserializeOwned>(
        &self,
        action: &str,
        request: T,
    ) -> Result<Result<R, OCPP1_6Error>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let message_id = Uuid::new_v4();
        let raw_payload = serde_json::to_string(&(2, message_id, action, request))?;

        info!(
            charger_id = self.charger_id,
            protocol = OcppProtocol::Ocpp1_6.to_string(),
            message_id = &message_id.to_string(),
            action = action,
            raw_payload = &raw_payload,
            "Sending call -->"
        );

        {
            let mut sink = self.use_sink().await?;
            sink.send(Text(raw_payload)).await?;
        }

        let (sender, receiver) = oneshot::channel();

        {
            let mut lock = self.ocpp_1_6_message_queue.lock().await;
            lock.insert(message_id.to_string(), sender);
        }

        let timeout_duration = Duration::from_secs(
            self.config
                .ocpp
                .clone()
                .unwrap_or_default()
                .message_timeout_secs
                .unwrap_or(30),
        );

        info!("Waiting for response");
        match timeout(timeout_duration, receiver).await?? {
            Ok(val) => {
                let result = serde_json::from_value(val)?;
                Ok(Ok(result))
            }
            Err(err) => Ok(Err(err)),
        }
    }
}

#[async_trait::async_trait]
impl NetworkInterfaceHandle for OcppNetworkInterfaceHandle {
    async fn disconnect(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut sink = self.use_sink().await?;
        sink.close().await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Ocpp16NetworkInterfaceHandle for OcppNetworkInterfaceHandle {
    async fn send_get_configuration(
        &self,
        request: GetConfigurationRequest,
    ) -> Result<
        Result<GetConfigurationResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send_ocpp_1_6("GetConfiguration", request).await
    }

    async fn send_change_configuration(
        &self,
        request: ChangeConfigurationRequest,
    ) -> Result<
        Result<ChangeConfigurationResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send_ocpp_1_6("ChangeConfiguration", request).await
    }

    async fn send_remote_start_transaction(
        &self,
        request: RemoteStartTransactionRequest,
    ) -> Result<
        Result<RemoteStartTransactionResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send_ocpp_1_6("RemoteStartTransaction", request).await
    }

    async fn send_remote_stop_transaction(
        &self,
        request: RemoteStopTransactionRequest,
    ) -> Result<
        Result<RemoteStopTransactionResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send_ocpp_1_6("RemoteStopTransaction", request).await
    }

    async fn send_trigger_message(
        &self,
        request: TriggerMessageRequest,
    ) -> Result<
        Result<TriggerMessageResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send_ocpp_1_6("TriggerMessage", request).await
    }

    async fn send_reset(
        &self,
        request: ResetRequest,
    ) -> Result<
        Result<ResetResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send_ocpp_1_6("Reset", request).await
    }

    async fn send_cancel_reservation(
        &self,
        request: CancelReservationRequest,
    ) -> Result<
        Result<CancelReservationResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send_ocpp_1_6("CancelReservation", request).await
    }

    async fn send_change_availability(
        &self,
        request: ChangeAvailabilityRequest,
    ) -> Result<
        Result<ChangeAvailabilityResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send_ocpp_1_6("ChangeAvailability", request).await
    }

    async fn send_reserve_now(
        &self,
        request: ReserveNowRequest,
    ) -> Result<Result<ReserveNowResponse, OCPP1_6Error>, Box<dyn Error + Send + Sync + 'static>>
    {
        self.send_ocpp_1_6("ReserveNow", request).await
    }
}

#[async_trait::async_trait]
impl Ocpp2_0_1NetworkInterfaceHandle for OcppNetworkInterfaceHandle {}
