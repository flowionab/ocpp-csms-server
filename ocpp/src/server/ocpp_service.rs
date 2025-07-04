use crate::charger::ChargerPool;
use crate::ocpp_csms_server::ocpp_server::Ocpp;
use crate::ocpp_csms_server::reboot_charger_request::RebootType;
use crate::ocpp_csms_server::{
    CancelOutletReservationRequest, CancelOutletReservationResponse,
    ChangeChargerAvailabilityRequest, ChangeChargerAvailabilityResponse,
    ChangeConnectorAvailabilityRequest, ChangeConnectorAvailabilityResponse,
    ChangeEvseAvailabilityRequest, ChangeEvseAvailabilityResponse,
    ChangeOcpp16configurationValueRequest, ChangeOcpp16configurationValueResponse,
    ClearChargerCacheRequest, ClearChargerCacheResponse, CreateRfidScanSessionRequest,
    CreateRfidScanSessionResponse, RebootChargerRequest, RebootChargerResponse,
    RfidScanSessionStatus, StartTransactionRequest, StartTransactionResponse,
    StopTransactionRequest, StopTransactionResponse,
};
use chrono::{TimeDelta, Utc};
use shared::ConnectorStatus;
use std::ops::Add;
use tonic::{Request, Response, Status};
use tracing::warn;
use uuid::Uuid;

#[derive(Clone)]
pub struct OcppService {
    charger_pool: ChargerPool,
}

impl OcppService {
    pub fn new(charger_pool: ChargerPool) -> Self {
        Self { charger_pool }
    }
}

#[tonic::async_trait]
impl Ocpp for OcppService {
    async fn start_transaction(
        &self,
        request: Request<StartTransactionRequest>,
    ) -> Result<Response<StartTransactionResponse>, Status> {
        let payload = request.into_inner();
        let evse_id = Uuid::parse_str(&payload.evse_id)
            .map_err(|_| Status::invalid_argument("Invalid evse_id"))?;

        match self.charger_pool.get(&payload.charger_id).await {
            Some(charger) => {
                let mut lock = charger.lock().await;
                let transaction = lock.start_transaction(evse_id).await?;
                Ok(Response::new(StartTransactionResponse {
                    transaction: Some(transaction.into()),
                }))
            }
            None => Err(Status::not_found(
                "A charger with this id is not connected to this instance",
            )),
        }
    }

    async fn reboot_charger(
        &self,
        request: Request<RebootChargerRequest>,
    ) -> Result<Response<RebootChargerResponse>, Status> {
        let payload = request.into_inner();
        match self.charger_pool.get(&payload.charger_id).await {
            Some(charger) => {
                let mut lock = charger.lock().await;
                match RebootType::try_from(payload.reboot_type).unwrap() {
                    RebootType::Soft => {
                        lock.reboot_soft().await?;
                    }
                    RebootType::Hard => {
                        lock.reboot_hard().await?;
                    }
                }
                Ok(Response::new(RebootChargerResponse {}))
            }
            None => Err(Status::not_found(
                "A charger with this id is not connected to this instance",
            )),
        }
    }

    async fn cancel_outlet_reservation(
        &self,
        request: Request<CancelOutletReservationRequest>,
    ) -> Result<Response<CancelOutletReservationResponse>, Status> {
        let payload = request.into_inner();
        match self.charger_pool.get(&payload.charger_id).await {
            Some(charger) => {
                let mut lock = charger.lock().await;
                lock.cancel_outlet_reservation(&payload.outlet_id).await?;

                Ok(Response::new(CancelOutletReservationResponse {}))
            }
            None => Err(Status::not_found(
                "A charger with this id is not connected to this instance",
            )),
        }
    }

    async fn change_charger_availability(
        &self,
        request: Request<ChangeChargerAvailabilityRequest>,
    ) -> Result<Response<ChangeChargerAvailabilityResponse>, Status> {
        let payload = request.into_inner();
        match self.charger_pool.get(&payload.charger_id).await {
            Some(charger) => {
                let mut lock = charger.lock().await;
                lock.change_charger_availability(payload.operative).await?;

                Ok(Response::new(ChangeChargerAvailabilityResponse {}))
            }
            None => Err(Status::not_found(
                "A charger with this id is not connected to this instance",
            )),
        }
    }

    async fn change_evse_availability(
        &self,
        request: Request<ChangeEvseAvailabilityRequest>,
    ) -> Result<Response<ChangeEvseAvailabilityResponse>, Status> {
        let payload = request.into_inner();
        match self.charger_pool.get(&payload.charger_id).await {
            Some(charger) => {
                let mut lock = charger.lock().await;
                lock.change_evse_availability(&payload.evse_id, payload.operative)
                    .await?;

                Ok(Response::new(ChangeEvseAvailabilityResponse {}))
            }
            None => Err(Status::not_found(
                "A charger with this id is not connected to this instance",
            )),
        }
    }

    async fn change_connector_availability(
        &self,
        request: Request<ChangeConnectorAvailabilityRequest>,
    ) -> Result<Response<ChangeConnectorAvailabilityResponse>, Status> {
        let payload = request.into_inner();
        let evse_id = Uuid::parse_str(&payload.evse_id)
            .map_err(|_| Status::invalid_argument("Invalid evse_id"))?;
        let connector_id = Uuid::parse_str(&payload.connector_id)
            .map_err(|_| Status::invalid_argument("Invalid connector_id"))?;
        match self.charger_pool.get(&payload.charger_id).await {
            Some(charger) => {
                let mut lock = charger.lock().await;
                lock.change_connector_availability(evse_id, connector_id, payload.operative)
                    .await?;

                Ok(Response::new(ChangeConnectorAvailabilityResponse {}))
            }
            None => Err(Status::not_found(
                "A charger with this id is not connected to this instance",
            )),
        }
    }

    async fn change_ocpp1_6configuration_value(
        &self,
        _request: Request<ChangeOcpp16configurationValueRequest>,
    ) -> Result<Response<ChangeOcpp16configurationValueResponse>, Status> {
        todo!()
    }

    async fn clear_charger_cache(
        &self,
        _request: Request<ClearChargerCacheRequest>,
    ) -> Result<Response<ClearChargerCacheResponse>, Status> {
        todo!()
    }

    async fn stop_transaction(
        &self,
        request: Request<StopTransactionRequest>,
    ) -> Result<Response<StopTransactionResponse>, Status> {
        let payload = request.into_inner();
        let transaction_id = Uuid::parse_str(&payload.transaction_id)
            .map_err(|_| Status::invalid_argument("Invalid transaction_id"))?;

        match self.charger_pool.get(&payload.charger_id).await {
            Some(charger) => {
                let mut lock = charger.lock().await;
                let transaction = lock.stop_transaction(transaction_id).await?;
                Ok(Response::new(StopTransactionResponse {
                    transaction: Some(transaction.into()),
                }))
            }
            None => Err(Status::not_found(
                "A charger with this id is not connected to this instance",
            )),
        }
    }

    async fn create_rfid_scan_session(
        &self,
        request: Request<CreateRfidScanSessionRequest>,
    ) -> Result<Response<CreateRfidScanSessionResponse>, Status> {
        let payload = request.into_inner();
        match self.charger_pool.get(&payload.charger_id).await {
            Some(charger) => {
                let mut lock = charger.lock().await;

                for evse in &lock.data.evses {
                    for connector in &evse.connectors {
                        if connector.status != ConnectorStatus::Available {
                            return Err(Status::failed_precondition(
                                "cannot create RFID scan session while connector is not available",
                            ));
                        }
                    }
                }

                let expires_at = Utc::now().add(TimeDelta::seconds(120));

                lock.reserve_charger(expires_at).await?;

                let session = lock
                    .data_store
                    .create_rfid_scan_session(&lock.id, expires_at)
                    .await
                    .map_err(|err| {
                        warn!(error_message = %err, "Failed to create RFID scan session");
                        Status::internal(format!("Failed to create RFID scan session: {}", err))
                    })?;

                Ok(Response::new(CreateRfidScanSessionResponse {
                    session: Some(session.into()),
                }))
            }
            None => Err(Status::not_found(
                "A charger with this id is not connected to this instance",
            )),
        }
    }
}

impl From<shared::Transaction> for crate::ocpp_csms_server::Transaction {
    fn from(value: shared::Transaction) -> Self {
        Self {
            id: value.id.to_string(),
            charger_id: value.charger_id,
            ocpp_transaction_id: value.ocpp_transaction_id,
            start_time: value.start_time.timestamp_millis(),
            end_time: value.end_time.map(|i| i.timestamp_millis()),
            watt_charged: value.watt_charged,
            is_authorized: value.is_authorized,
        }
    }
}

impl From<shared::RfidScanSession> for crate::ocpp_csms_server::RfidScanSession {
    fn from(session: shared::RfidScanSession) -> Self {
        let status = if session.tag_scanned_at.is_some() {
            RfidScanSessionStatus::Completed
        } else if session.expires_at > chrono::Utc::now() {
            RfidScanSessionStatus::Active
        } else {
            RfidScanSessionStatus::Failed
        };
        Self {
            id: session.id.to_string(),
            charger_id: session.charger_id,
            rfid_uid_hex: session.rfid_uid_hex,
            expires_at: session.expires_at.timestamp_millis() as u64,
            status: status.into(),
            created_at: session.created_at.timestamp_millis() as u64,
        }
    }
}
