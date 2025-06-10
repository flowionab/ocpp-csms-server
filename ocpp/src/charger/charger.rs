use crate::charger::charger_model::ChargerModel;
use crate::event::EventManager;
use crate::network_interface::ProtocolHandle;
use crate::server::map_ocpp1_6_error_to_status;
use chrono::Utc;
use rand::Rng;
use rust_ocpp::v1_6::messages::cancel_reservation::CancelReservationRequest;
use rust_ocpp::v1_6::messages::change_availability::ChangeAvailabilityRequest;
use rust_ocpp::v1_6::messages::change_configuration::ChangeConfigurationRequest;
use rust_ocpp::v1_6::messages::remote_start_transaction::RemoteStartTransactionRequest;
use rust_ocpp::v1_6::messages::remote_stop_transaction::RemoteStopTransactionRequest;
use rust_ocpp::v1_6::messages::reset::ResetRequest;
use rust_ocpp::v1_6::types::{
    AvailabilityStatus, AvailabilityType, CancelReservationStatus, ConfigurationStatus,
    RemoteStartStopStatus, ResetRequestStatus, ResetResponseStatus,
};
use shared::{ChargerData, Config};
use shared::{DataStore, Transaction};
use std::sync::Arc;
use tonic::Status;
use tracing::{error, info, warn};
use uuid::Uuid;

pub struct Charger {
    pub id: String,

    pub handle: ProtocolHandle,

    /// The shared configuration for all chargers
    pub config: Arc<Config>,
    pub data_store: Arc<dyn DataStore + Send + Sync>,
    pub authenticated: bool,

    // This is a cached version of the data, perhaps it is redundant
    pub data: ChargerData,

    pub password: Option<String>,

    pub node_address: String,

    pub easee_master_password: Option<String>,

    pub event_manager: EventManager,
}

impl Charger {
    pub async fn setup(
        id: &str,
        config: Arc<Config>,
        handle: ProtocolHandle,
        data_store: Arc<dyn DataStore + Send + Sync>,
        node_address: &str,
        easee_master_password: Option<String>,
        event_manager: EventManager,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let data = data_store.get_charger_data_by_id(id).await?;

        let authenticated = false;

        Ok(Self {
            data_store,
            id: id.to_string(),
            handle,
            authenticated,
            config: config.clone(),
            data: data.unwrap_or_else(|| ChargerData::new(id, &config)),
            password: None,
            node_address: node_address.to_string(),
            easee_master_password,
            event_manager,
        })
    }

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

    pub async fn sync_data(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.data_store.save_charger_data(&self.data).await?;
        Ok(())
    }

    pub async fn start_transaction(&mut self, evse_id: Uuid) -> Result<Transaction, Status> {
        let evse = self
            .data
            .evse(evse_id)
            .ok_or_else(|| Status::not_found("Evse not found"))?;
        let evse_id = evse.id;

        match &self.handle {
            ProtocolHandle::Ocpp1_6(handle) => {
                let ocpp_evse_id = evse.ocpp_evse_id;
                let response = handle
                    .send_remote_start_transaction(RemoteStartTransactionRequest {
                        connector_id: Some(ocpp_evse_id),
                        id_tag: "central".to_string(),
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
                    RemoteStartStopStatus::Accepted => {
                        let transaction = self
                            .data_store
                            .get_ongoing_transaction(&self.id, evse_id)
                            .await
                            .map_err(|error| {
                                error!(
                                    error_message = error.to_string(),
                                    "Failed to get ongoing transaction, due to internal error"
                                );
                                Status::internal(
                                    "Failed to get ongoing transaction, due to internal error",
                                )
                            })?;

                        match transaction {
                            None => {
                                let transaction_id: i32 = {
                                    let mut rng = rand::rng();
                                    rng.random::<i32>()
                                };
                                let transaction = self
                                    .data_store
                                    .create_transaction(
                                        &self.id,
                                        evse_id,
                                        &transaction_id.to_string(),
                                        Utc::now(),
                                        true,
                                    )
                                    .await
                                    .map_err(|error| {
                                        error!(
                                            error_message = error.to_string(),
                                            "Failed to create transaction, due to internal error"
                                        );
                                        Status::internal(
                                            "Failed to create transaction, due to internal error",
                                        )
                                    })?;
                                Ok(transaction)
                            }
                            Some(transaction) => {
                                self.data_store
                                    .update_transaction_is_authorized(transaction.id, true)
                                    .await
                                    .map_err(|error| {
                                        error!(
                                            error_message = error.to_string(),
                                            "Failed to update transaction, due to internal error"
                                        );
                                        Status::internal(
                                            "Failed to update transaction, due to internal error",
                                        )
                                    })?;
                                Ok(transaction)
                            }
                        }
                    }
                    RemoteStartStopStatus::Rejected => {
                        Err(Status::cancelled("Charger could not start transaction"))
                    }
                }
            }
            ProtocolHandle::Ocpp2_0_1(_handle) => {
                Err(Status::internal("We can't handle ocpp 2.0.1 yet"))
            }
        }
    }

    pub async fn stop_transaction(&mut self, transaction_id: Uuid) -> Result<Transaction, Status> {
        let transaction = self
            .data_store
            .get_transaction(transaction_id)
            .await
            .map_err(|error| {
                error!(
                    error_message = error.to_string(),
                    "Failed to get transaction, due to internal error"
                );
                Status::internal("Failed to get transaction, due to internal error")
            })?
            .ok_or_else(|| {
                error!("Transaction not found");
                Status::not_found("Transaction not found")
            })?;

        match &self.handle {
            ProtocolHandle::Ocpp1_6(handle) => {
                let response = handle
                    .send_remote_stop_transaction(RemoteStopTransactionRequest {
                        transaction_id: transaction.ocpp_transaction_id.parse().map_err(
                            |_error| {
                                error!("Failed to parse transaction id");
                                Status::internal("Failed to parse transaction id")
                            },
                        )?,
                    })
                    .await
                    .map_err(|error| {
                        error!(
                            error_message = error.to_string(),
                            "Failed to stop transaction, due to internal error"
                        );
                        Status::internal("Failed to stop transaction, due to internal error")
                    })?
                    .map_err(map_ocpp1_6_error_to_status)?;

                match response.status {
                    RemoteStartStopStatus::Accepted => Ok(transaction),
                    RemoteStartStopStatus::Rejected => {
                        Err(Status::cancelled("Charger could not start transaction"))
                    }
                }
            }
            ProtocolHandle::Ocpp2_0_1(_handle) => {
                Err(Status::internal("We can't handle ocpp 2.0.1 yet"))
            }
        }
    }

    pub async fn reboot_soft(&mut self) -> Result<(), Status> {
        match &self.handle {
            ProtocolHandle::Ocpp1_6(handle) => {
                let response = handle
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
            ProtocolHandle::Ocpp2_0_1(_handle) => {
                Err(Status::internal("We can't handle ocpp 2.0.1 yet"))
            }
        }
    }

    pub async fn reboot_hard(&mut self) -> Result<(), Status> {
        match &self.handle {
            ProtocolHandle::Ocpp1_6(handle) => {
                let response = handle
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
            ProtocolHandle::Ocpp2_0_1(_handle) => {
                Err(Status::internal("We can't handle ocpp 2.0.1 yet"))
            }
        }
    }

    pub async fn cancel_outlet_reservation(&mut self, _outlet_id: &str) -> Result<(), Status> {
        match &self.handle {
            ProtocolHandle::Ocpp1_6(handle) => {
                let response = handle
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
            ProtocolHandle::Ocpp2_0_1(_handle) => {
                Err(Status::internal("We can't handle ocpp 2.0.1 yet"))
            }
        }
    }

    pub async fn change_charger_availability(&mut self, available: bool) -> Result<(), Status> {
        match &self.handle {
            ProtocolHandle::Ocpp1_6(handle) => {
                let response = handle
                    .send_change_availability(ChangeAvailabilityRequest {
                        connector_id: 0,
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
            ProtocolHandle::Ocpp2_0_1(_handle) => {
                Err(Status::internal("We can't handle ocpp 2.0.1 yet"))
            }
        }
    }

    pub async fn change_evse_availability(
        &mut self,
        evse_id: &str,
        available: bool,
    ) -> Result<(), Status> {
        let evse = self
            .data
            .evses
            .clone()
            .into_iter()
            .find(|i| i.id.to_string() == evse_id)
            .ok_or_else(|| Status::not_found("Evse not found"))?;

        match &self.handle {
            ProtocolHandle::Ocpp1_6(handle) => {
                let response = handle
                    .send_change_availability(ChangeAvailabilityRequest {
                        connector_id: evse.ocpp_evse_id,
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
            ProtocolHandle::Ocpp2_0_1(_handle) => {
                Err(Status::internal("We can't handle ocpp 2.0.1 yet"))
            }
        }
    }

    pub async fn change_connector_availability(
        &mut self,
        evse_id: Uuid,
        connector_id: Uuid,
        available: bool,
    ) -> Result<(), Status> {
        let evse = self
            .data
            .evses
            .clone()
            .into_iter()
            .find(|i| i.id == evse_id)
            .ok_or_else(|| Status::not_found("Evse not found"))?;

        let _connector = evse
            .connectors
            .clone()
            .into_iter()
            .find(|i| i.id == connector_id)
            .ok_or_else(|| Status::not_found("Connector not found"))?;

        match &self.handle {
            ProtocolHandle::Ocpp1_6(handle) => {
                let response = handle
                    .send_change_availability(ChangeAvailabilityRequest {
                        connector_id: evse.ocpp_evse_id,
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
            ProtocolHandle::Ocpp2_0_1(_handle) => {
                Err(Status::internal("We can't handle ocpp 2.0.1 yet"))
            }
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

    pub async fn validate_rfid_tag(
        &self,
        tag: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if tag == "central" {
            return Ok(true);
        }

        let rfid_tag = self.data_store.get_rfid_tag_by_hex(tag).await?;

        Ok(rfid_tag.is_some())
    }

    pub(crate) async fn get_ongoing_transaction(
        &mut self,
        evse_ocpp_id: u32,
    ) -> Result<Option<Transaction>, Box<dyn std::error::Error + Send + Sync>> {
        let evse = self.data.evse_by_ocpp_id_or_create(evse_ocpp_id);
        self.data_store
            .get_ongoing_transaction(&self.id, evse.id)
            .await
    }

    pub(crate) async fn end_ongoing_transaction(
        &mut self,
        connector_id: u32,
        end_time: &Option<chrono::DateTime<Utc>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(transaction) = self.get_ongoing_transaction(connector_id).await? {
            info!(
                "Ending transaction {} for connector {}",
                transaction.ocpp_transaction_id, connector_id
            );
            self.data_store
                .end_transaction(
                    &self.id,
                    &transaction.ocpp_transaction_id,
                    end_time.unwrap_or_else(Utc::now),
                )
                .await?
        }

        Ok(())
    }

    pub(crate) async fn update_ocpp_1_6_charger_configuration(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        if let Some(configuration) = &self.data.ocpp1_6configuration {
            let target_configuration = self
                .data
                .settings
                .get_ocpp_1_6_configuration_entries(&self.config);

            let mut needs_reboot = false;
            for (key, value) in target_configuration {
                if let Some(config_value) = configuration.get_configuration(&key) {
                    if !config_value.read_only && config_value.value != Some(value.clone()) {
                        match self
                            .handle
                            .as_ocpp1_6()
                            .unwrap()
                            .send_change_configuration(ChangeConfigurationRequest {
                                key: key.to_string(),
                                value: value.to_string(),
                            })
                            .await?
                        {
                            Ok(response) => {
                                if response.status == ConfigurationStatus::Accepted
                                    || response.status == ConfigurationStatus::RebootRequired
                                {
                                    info!("Configuration {} updated to {}", key, value);
                                    if response.status == ConfigurationStatus::RebootRequired {
                                        needs_reboot = true;
                                    }
                                } else {
                                    warn!(
                                        "Failed to update configuration {}: {:?}",
                                        key, response.status
                                    );
                                }
                            }
                            Err(err) => {
                                warn!(
                                    error_message = err.to_string(),
                                    "Failed to change configuration"
                                );
                            }
                        }
                    }
                }
            }
            if needs_reboot {
                match self
                    .handle
                    .as_ocpp1_6()
                    .unwrap()
                    .send_reset(ResetRequest {
                        kind: ResetRequestStatus::Soft,
                    })
                    .await?
                {
                    Ok(response) => {
                        if response.status == ResetResponseStatus::Accepted {
                            info!("Charger rebooted successfully");
                        } else {
                            warn!("Failed to reboot the charger");
                        }
                    }
                    Err(err) => {
                        warn!(
                            error_message = err.to_string(),
                            "Failed to reboot the charger"
                        );
                    }
                }
            }
        }
        Ok(())
    }
}
