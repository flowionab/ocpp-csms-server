use crate::charger::charger_model::ChargerModel;
use crate::charger::ocpp1_6interface::Ocpp1_6Interface;
use crate::charger::ocpp_2_0_interface::Ocpp2_0_1Interface;
use crate::ocpp::OcppProtocol;
use crate::server::map_ocpp1_6_error_to_status;
use futures::stream::SplitSink;
use ocpp_client::ocpp_1_6::OCPP1_6Error;
use poem::http::StatusCode;
use poem::web::websocket::{Message, WebSocketStream};
use poem::Response;
use rust_ocpp::v1_6::messages::cancel_reservation::CancelReservationRequest;
use rust_ocpp::v1_6::messages::change_availability::ChangeAvailabilityRequest;
use rust_ocpp::v1_6::messages::remote_start_transaction::RemoteStartTransactionRequest;
use rust_ocpp::v1_6::messages::reset::ResetRequest;
use rust_ocpp::v1_6::types::{
    AvailabilityStatus, AvailabilityType, CancelReservationStatus, RemoteStartStopStatus,
    ResetRequestStatus, ResetResponseStatus,
};
use serde_json::Value;
use shared::DataStore;
use shared::{ChargerData, Config};
use std::collections::BTreeMap;
use std::env;
use std::sync::Arc;
use tokio::sync::oneshot::Sender;
use tokio::sync::Mutex;
use tonic::Status;
use tracing::{error, instrument, warn};

type Ocpp1_6MessageQueue = Arc<Mutex<BTreeMap<String, Sender<Result<Value, OCPP1_6Error>>>>>;

#[derive(Clone)]
pub struct Charger {
    pub id: String,

    #[allow(dead_code)]
    pub config: Config,
    pub data_store: Arc<dyn DataStore>,
    pub authenticated: bool,

    pub data: ChargerData,

    pub password: Option<String>,

    pub protocol: Option<OcppProtocol>,
    pub sink: Option<Arc<Mutex<SplitSink<WebSocketStream, Message>>>>,

    pub message_queue: Ocpp1_6MessageQueue,

    pub node_address: String,
}

impl Charger {
    #[instrument(skip_all)]
    pub async fn setup(
        id: &str,
        config: &Config,
        data_store: Arc<dyn DataStore>,
        message_queue: Ocpp1_6MessageQueue,
    ) -> Result<Self, Response> {
        let data = data_store.get_charger_data_by_id(id).await.map_err(|e| {
            error!(
                error_message = e.to_string(),
                "Could not retrieve charger data"
            );
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Could not retrieve charger data".to_string())
        })?;

        let authenticated = config
            .ocpp
            .clone()
            .unwrap_or_default()
            .disable_charger_auth
            .unwrap_or_default();

        Ok(Self {
            data_store,
            id: id.to_string(),
            authenticated,
            config: config.clone(),
            data: data.unwrap_or_else(|| shared::ChargerData {
                id: id.to_string(),
                ..Default::default()
            }),
            password: None,
            protocol: None,
            sink: None,
            message_queue,
            node_address: "http://localhost:50052".to_string(),
        })
    }

    #[instrument(skip(self))]
    pub async fn ping(&mut self) {
        if let Err(error) = self
            .data_store
            .update_charger_connection_info(&self.id, true, &self.node_address)
            .await
        {
            error!(
                error_message = error.to_string(),
                "Failed to update charger info"
            );
        }
    }

    pub fn set_protocol(&mut self, protocol: OcppProtocol) {
        self.protocol = Some(protocol)
    }

    pub fn attach_sink(&mut self, sink: Arc<Mutex<SplitSink<WebSocketStream, Message>>>) {
        self.sink = Some(sink)
    }

    #[instrument(skip_all)]
    pub async fn sync_data(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.data_store.save_charger_data(&self.data).await?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub async fn authenticate_with_password(
        &mut self,
        password: Option<String>,
    ) -> Result<(), Response> {
        self.password = password.clone();

        if let Some(ChargerModel::Easee(_)) = self.model() {
            return match env::var("EASEE_MASTER_PASSWORD") {
                Ok(master_password) => {
                    if password == Some(master_password) {
                        self.authenticated = true;
                        Ok(())
                    } else {
                        warn!(
                            charger_id = self.id.to_string(),
                            "The charger is an Easee charger, but the password is incorrect"
                        );
                        Err(Response::builder()
                            .status(StatusCode::FORBIDDEN)
                            .body("Invalid password".to_string()))
                    }
                }
                Err(_) => {
                    warn!(
                        charger_id = self.id.to_string(),
                        "The charger is an Easee charger, but the EASEE_MASTER_PASSWORD env var is not set, will reject it for now"
                    );
                    Err(Response::builder()
                        .status(StatusCode::FORBIDDEN)
                        .body("Easee charger are currently not accepted".to_string()))
                }
            };
        }

        let hashed_password_opt = self.data_store.get_password(&self.id).await.map_err(|e| {
            error!(
                error_message = e.to_string(),
                "Failed to validate credentials"
            );
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to validate credentials".to_string())
        })?;
        match &hashed_password_opt {
            Some(hashed_password) => match &password {
                None => {
                    warn!("Missing credentials");
                    Err(Response::builder()
                        .status(StatusCode::FORBIDDEN)
                        .body("Missing credentials".to_string()))
                }
                Some(p) => {
                    let result = bcrypt::verify(p, hashed_password).map_err(|_e| {
                        error!("Failed to validate credentials");
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body("Failed to validate credentials".to_string())
                    })?;

                    match result {
                        true => {
                            self.authenticated = true;
                            Ok(())
                        }
                        false => {
                            warn!("Invalid credentials");
                            Err(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body("Invalid credentials".to_string()))
                        }
                    }
                }
            },
            None => {
                self.authenticated = false;
                if password.is_some() {
                    warn!(charger_id = self.id.to_string(), "The charger does have existing credentials, but it has not been onborded yet to our system, ignoring the credentials for now...")
                }
                Ok(())
            }
        }
    }

    pub fn ocpp1_6(&mut self) -> Ocpp1_6Interface {
        Ocpp1_6Interface::new(self)
    }

    #[allow(dead_code)]
    pub fn ocpp2_0_1(&mut self) -> Ocpp2_0_1Interface {
        Ocpp2_0_1Interface::new(self)
    }

    #[instrument(skip_all)]
    pub async fn start_transaction(&mut self, outlet_id: &str) -> Result<(), Status> {
        let outlet = self
            .data
            .outlets
            .clone()
            .into_iter()
            .find(|i| i.id.to_string() == outlet_id)
            .ok_or_else(|| Status::not_found("Outlet not found"))?;

        match self.protocol {
            None => Err(Status::failed_precondition(
                "The charger has not picked a ocpp protocol yet",
            )),
            Some(protocol) => match protocol {
                OcppProtocol::Ocpp1_6 => {
                    let response = self
                        .ocpp1_6()
                        .send_remote_start_transaction(RemoteStartTransactionRequest {
                            connector_id: Some(outlet.ocpp_connector_id),
                            id_tag: "".to_string(),
                            charging_profile: None,
                        })
                        .await
                        .map_err(|error| {
                            error!(
                                error_message = error.to_string(),
                                "Failed to start transaction, due to internal error"
                            );
                            Status::internal("Failed to start transaction, due to internal error")
                        })?
                        .map_err(map_ocpp1_6_error_to_status)?;

                    match response.status {
                        RemoteStartStopStatus::Accepted => Ok(()),
                        RemoteStartStopStatus::Rejected => {
                            Err(Status::cancelled("Charger could not start transaction"))
                        }
                    }
                }
                OcppProtocol::Ocpp2_0_1 => Err(Status::internal("We can't handle ocpp 2.0.1 yet")),
            },
        }
    }

    #[instrument(skip_all)]
    pub async fn reboot_soft(&mut self) -> Result<(), Status> {
        match self.protocol {
            None => Err(Status::failed_precondition(
                "The charger has not picked a ocpp protocol yet",
            )),
            Some(protocol) => match protocol {
                OcppProtocol::Ocpp1_6 => {
                    let response = self
                        .ocpp1_6()
                        .send_reset(ResetRequest {
                            kind: ResetRequestStatus::Soft,
                        })
                        .await
                        .map_err(|error| {
                            error!(
                                error_message = error.to_string(),
                                "Failed to reset charger due to internal error"
                            );
                            Status::internal("Failed to reset, due to internal error")
                        })?
                        .map_err(map_ocpp1_6_error_to_status)?;

                    match response.status {
                        ResetResponseStatus::Accepted => Ok(()),
                        ResetResponseStatus::Rejected => {
                            Err(Status::cancelled("Charger could not reset"))
                        }
                    }
                }
                OcppProtocol::Ocpp2_0_1 => Err(Status::internal("We can't handle ocpp 2.0.1 yet")),
            },
        }
    }

    #[instrument(skip_all)]
    pub async fn reboot_hard(&mut self) -> Result<(), Status> {
        match self.protocol {
            None => Err(Status::failed_precondition(
                "The charger has not picked a ocpp protocol yet",
            )),
            Some(protocol) => match protocol {
                OcppProtocol::Ocpp1_6 => {
                    let response = self
                        .ocpp1_6()
                        .send_reset(ResetRequest {
                            kind: ResetRequestStatus::Hard,
                        })
                        .await
                        .map_err(|error| {
                            error!(
                                error_message = error.to_string(),
                                "Failed to reset charger due to internal error"
                            );
                            Status::internal("Failed to reset, due to internal error")
                        })?
                        .map_err(map_ocpp1_6_error_to_status)?;

                    match response.status {
                        ResetResponseStatus::Accepted => Ok(()),
                        ResetResponseStatus::Rejected => {
                            Err(Status::cancelled("Charger could not reset"))
                        }
                    }
                }
                OcppProtocol::Ocpp2_0_1 => Err(Status::internal("We can't handle ocpp 2.0.1 yet")),
            },
        }
    }

    #[instrument(skip_all)]
    pub async fn cancel_outlet_reservation(&mut self, _outlet_id: &str) -> Result<(), Status> {
        match self.protocol {
            None => Err(Status::failed_precondition(
                "The charger has not picked a ocpp protocol yet",
            )),
            Some(protocol) => match protocol {
                OcppProtocol::Ocpp1_6 => {
                    let response = self
                        .ocpp1_6()
                        .send_cancel_reservation(CancelReservationRequest { reservation_id: 0 })
                        .await
                        .map_err(|error| {
                            error!(
                                error_message = error.to_string(),
                                "Failed to cancel reservation due to internal error"
                            );
                            Status::internal("Failed to cancel reservation, due to internal error")
                        })?
                        .map_err(map_ocpp1_6_error_to_status)?;

                    match response.status {
                        CancelReservationStatus::Accepted => Ok(()),
                        CancelReservationStatus::Rejected => {
                            Err(Status::cancelled("Charger could not cancel reservation"))
                        }
                    }
                }
                OcppProtocol::Ocpp2_0_1 => Err(Status::internal("We can't handle ocpp 2.0.1 yet")),
            },
        }
    }

    #[instrument(skip_all)]
    pub async fn change_availability(
        &mut self,
        outlet_id: &str,
        available: bool,
    ) -> Result<(), Status> {
        let protocol = self.protocol.ok_or_else(|| {
            Status::failed_precondition("The charger has not picked a ocpp protocol yet")
        })?;

        let outlet = self
            .data
            .outlets
            .clone()
            .into_iter()
            .find(|i| i.id.to_string() == outlet_id)
            .ok_or_else(|| Status::not_found("Outlet not found"))?;

        match protocol {
            OcppProtocol::Ocpp1_6 => {
                let response = self
                    .ocpp1_6()
                    .send_change_availability(ChangeAvailabilityRequest {
                        connector_id: outlet.ocpp_connector_id,
                        kind: if available {
                            AvailabilityType::Operative
                        } else {
                            AvailabilityType::Inoperative
                        },
                    })
                    .await
                    .map_err(|error| {
                        error!(
                            error_message = error.to_string(),
                            "Failed to change availability due to internal error"
                        );
                        Status::internal("Failed to change availability, due to internal error")
                    })?
                    .map_err(map_ocpp1_6_error_to_status)?;

                match response.status {
                    AvailabilityStatus::Accepted => Ok(()),
                    AvailabilityStatus::Scheduled => Ok(()),
                    AvailabilityStatus::Rejected => {
                        Err(Status::cancelled("Charger could not change availability"))
                    }
                }
            }
            OcppProtocol::Ocpp2_0_1 => Err(Status::internal("We can't handle ocpp 2.0.1 yet")),
        }
    }

    pub fn model(&self) -> Option<ChargerModel> {
        if let Some(vendor) = &self.data.vendor {
            if let Some(model) = &self.data.model {
                return Some(ChargerModel::from_vendor_and_model(vendor, model));
            };
        };
        None
    }
}
