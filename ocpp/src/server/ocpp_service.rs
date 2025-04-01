use crate::charger::ChargerPool;
use crate::ocpp_csms_server::ocpp_server::Ocpp;
use crate::ocpp_csms_server::reboot_charger_request::RebootType;
use crate::ocpp_csms_server::{
    CancelOutletReservationRequest, CancelOutletReservationResponse,
    ChangeChargerAvailabilityRequest, ChangeChargerAvailabilityResponse,
    ChangeConnectorAvailabilityRequest, ChangeConnectorAvailabilityResponse,
    ChangeEvseAvailabilityRequest, ChangeEvseAvailabilityResponse,
    ChangeOcpp16configurationValueRequest, ChangeOcpp16configurationValueResponse,
    ClearChargerCacheRequest, ClearChargerCacheResponse, RebootChargerRequest,
    RebootChargerResponse, StartTransactionRequest, StartTransactionResponse,
    StopTransactionRequest, StopTransactionResponse,
};
use tonic::{Request, Response, Status};

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
        match self.charger_pool.get(&payload.charger_id).await {
            Some(charger) => {
                let mut lock = charger.lock().await;
                lock.start_transaction(&payload.outlet_id).await?;
                Ok(Response::new(StartTransactionResponse {
                    transaction_id: "".to_string(),
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
        match self.charger_pool.get(&payload.charger_id).await {
            Some(charger) => {
                let mut lock = charger.lock().await;
                lock.change_connector_availability(
                    &payload.evse_id,
                    &payload.connector_id,
                    payload.operative,
                )
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
        _request: Request<StopTransactionRequest>,
    ) -> Result<Response<StopTransactionResponse>, Status> {
        todo!()
    }
}
