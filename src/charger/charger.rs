use std::collections::BTreeMap;
use std::sync::Arc;
use futures::stream::SplitSink;
use ocpp_client::ocpp_1_6::OCPP1_6Error;
use poem::http::StatusCode;
use poem::Response;
use poem::web::websocket::{Message, WebSocketStream};
use serde_json::Value;
use tokio::sync::Mutex;
use tokio::sync::oneshot::Sender;
use tracing::{error, warn};
use crate::charger::charger_data::ChargerData;
use crate::charger::ocpp1_6interface::Ocpp1_6Interface;
use crate::data::DataStore;
use crate::ocpp::OcppProtocol;

#[derive(Clone)]
pub struct Charger {
    pub id: String,
    pub data_store: Arc<dyn DataStore>,
    pub authenticated: bool,

    pub data: ChargerData,

    pub password: Option<String>,

    pub protocol: Option<OcppProtocol>,
    pub sink: Option<Arc<Mutex<SplitSink<WebSocketStream, Message>>>>,

    pub message_queue: Arc<Mutex<BTreeMap<String, Sender<Result<Value, OCPP1_6Error>>>>>
}

impl Charger {
    pub async fn setup(id: &str, data_store: Arc<dyn DataStore>, message_queue: Arc<Mutex<BTreeMap<String, Sender<Result<Value, OCPP1_6Error>>>>>) -> Result<Self, Response> {
        let data = data_store.get_charger_data_by_id(id).await.map_err(|_e| {
            error!("Could not retrieve charger data");
            Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(
                "Could not retrieve charger data".to_string(),
            )
        })?;

        Ok(Self {
            data_store,
            id: id.to_string(),
            authenticated: false,
            data: data.unwrap_or_default(),
            password: None,
            protocol: None,
            sink: None,
            message_queue
        })
    }

    pub fn set_protocol(&mut self, protocol: OcppProtocol) {
        self.protocol = Some(protocol)
    }

    pub fn attach_sink(&mut self, sink: Arc<Mutex<SplitSink<WebSocketStream, Message>>>) {
        self.sink = Some(sink)
    }

    pub async fn sync_data(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.data_store.save_charger_data(&self.data).await?;
        Ok(())
    }

    pub async fn authenticate_with_password(&mut self, password: Option<String>) -> Result<(), Response> {
        self.password = password.clone();
        let hashed_password_opt = self.data_store.get_password(&self.id).await.map_err(|_e| {
            error!("Failed to validate credentials");
            Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(
                "Failed to validate credentials".to_string(),
            )
        })?;
        match &hashed_password_opt {
            Some(hashed_password) => {
                match &password {
                    None => {
                        warn!("Missing credentials");
                        Err(Response::builder().status(StatusCode::FORBIDDEN).body(
                            "Missing credentials".to_string(),
                        ))
                    }
                    Some(p) => {
                        let result = bcrypt::verify(&p, &hashed_password).map_err(|_e| {
                            error!("Failed to validate credentials");
                            Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(
                                "Failed to validate credentials".to_string(),
                            )
                        })?;

                        match result {
                            true => {
                                self.authenticated = true;
                                Ok(())
                            }
                            false => {
                                warn!("Invalid credentials");
                                Err(Response::builder().status(StatusCode::FORBIDDEN).body(
                                    "Invalid credentials".to_string(),
                                ))
                            }
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
}