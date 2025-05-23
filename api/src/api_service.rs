use crate::ocpp_csms_server::api_server::Api;
use crate::ocpp_csms_server::ocpp_client::OcppClient;
use crate::ocpp_csms_server::{
    CancelOutletReservationRequest, CancelOutletReservationResponse,
    ChangeChargerAvailabilityRequest, ChangeChargerAvailabilityResponse,
    ChangeConnectorAvailabilityRequest, ChangeConnectorAvailabilityResponse,
    ChangeEvseAvailabilityRequest, ChangeEvseAvailabilityResponse,
    ChangeOcpp16configurationValueRequest, ChangeOcpp16configurationValueResponse, Charger,
    ChargerSummary, ClearChargerCacheRequest, ClearChargerCacheResponse, CreateChargerRequest,
    CreateChargerResponse, Evse, GetChargerRequest, GetChargerResponse, GetChargersRequest,
    GetChargersResponse, Ocpp16configuration, RebootChargerRequest, RebootChargerResponse,
    StartTransactionRequest, StartTransactionResponse, StopTransactionRequest,
    StopTransactionResponse,
};
use shared::{ChargerConnectionInfo, DataStore};
use tokio::try_join;
use tonic::transport::Channel;
use tonic::{Request, Response, Status};
use tracing::{error, instrument, warn};

#[derive(Debug)]
pub struct ApiService {
    data_store: Box<dyn DataStore>,
}

impl ApiService {
    pub fn new(data_store: Box<dyn DataStore>) -> Self {
        Self { data_store }
    }

    async fn get_client(&self, charger_id: &str) -> Result<OcppClient<Channel>, Status> {
        let info: ChargerConnectionInfo = self
            .data_store
            .get_charger_connection_info(charger_id)
            .await
            .map_err(|error| {
                error!(
                    error_message = error.to_string(),
                    "could not get charger connection info"
                );
                Status::internal("Could not get charger connection info")
            })?
            .ok_or_else(|| {
                warn!("could not find charger connection info");
                Status::not_found("Could not find charger connection info")
            })?;

        let grpc_client = OcppClient::connect(info.node_address.clone())
            .await
            .map_err(|error| {
                error!(
                    node_address = &info.node_address,
                    error_message = error.to_string(),
                    "could not connect to ocpp node"
                );
                Status::internal("Could not connect to ocpp node")
            })?;
        Ok(grpc_client)
    }
}

#[tonic::async_trait]
impl Api for ApiService {
    async fn create_charger(
        &self,
        request: Request<CreateChargerRequest>,
    ) -> Result<Response<CreateChargerResponse>, Status> {
        let payload = request.into_inner();

        self.data_store
            .create_charger(&payload.charger_id)
            .await
            .map_err(|error| {
                if error.to_string().contains("duplicate key") {
                    warn!(
                        error_message = error.to_string(),
                        charger_id = payload.charger_id,
                        "charger already exists"
                    );
                    return Status::already_exists("Charger already exists");
                }
                error!(
                    error_message = error.to_string(),
                    charger_id = payload.charger_id,
                    "could not create charger"
                );
                Status::internal("Could not create charger")
            })?;

        let (charger, connection_info) = try_join!(
            self.data_store.get_charger_data_by_id(&payload.charger_id),
            self.data_store
                .get_charger_connection_info(&payload.charger_id)
        )
        .map_err(|error| {
            error!(error_message = error.to_string(), "could not get charger");
            Status::internal("Could not get charger")
        })?;

        let connection_info = connection_info.unwrap_or_default();

        Ok(Response::new(CreateChargerResponse {
            charger: charger.map(|charger| (charger, connection_info).into()),
        }))
    }

    #[instrument]
    async fn get_charger(
        &self,
        request: Request<GetChargerRequest>,
    ) -> Result<Response<GetChargerResponse>, Status> {
        let payload = request.into_inner();

        let (charger, connection_info) = try_join!(
            self.data_store.get_charger_data_by_id(&payload.charger_id),
            self.data_store
                .get_charger_connection_info(&payload.charger_id)
        )
        .map_err(|error| {
            error!(error_message = error.to_string(), "could not get charger");
            Status::internal("Could not get charger")
        })?;

        let connection_info = connection_info.unwrap_or_default();

        Ok(Response::new(GetChargerResponse {
            charger: charger.map(|charger| (charger, connection_info).into()),
        }))
    }

    #[instrument]
    async fn get_chargers(
        &self,
        request: Request<GetChargersRequest>,
    ) -> Result<Response<GetChargersResponse>, Status> {
        let payload = request.into_inner();
        let chargers = self
            .data_store
            .get_chargers(payload.page, payload.page_size)
            .await
            .map_err(|error| {
                error!(error_message = error.to_string(), "could not get chargers");
                Status::internal("Could not get chargers")
            })?;

        let count = self.data_store.count_chargers().await.map_err(|error| {
            error!(
                error_message = error.to_string(),
                "could not count chargers"
            );
            Status::internal("Could not count chargers")
        })?;

        let has_next = payload.page < (count % payload.page_size);

        Ok(Response::new(GetChargersResponse {
            chargers: chargers
                .into_iter()
                .map(|charger| ChargerSummary {
                    id: charger.id,
                    serial_number: charger.serial_number,
                    model: charger.model,
                    vendor: charger.vendor,
                })
                .collect(),
            page: payload.page,
            total_count: count,
            has_next,
            has_prev: payload.page > 1,
        }))
    }

    #[instrument]
    async fn reboot_charger(
        &self,
        request: Request<RebootChargerRequest>,
    ) -> Result<Response<RebootChargerResponse>, Status> {
        let payload = request.into_inner();
        let mut client = self.get_client(&payload.charger_id).await?;
        client.reboot_charger(payload).await
    }

    #[instrument]
    async fn cancel_outlet_reservation(
        &self,
        request: Request<CancelOutletReservationRequest>,
    ) -> Result<Response<CancelOutletReservationResponse>, Status> {
        let payload = request.into_inner();
        let mut client = self.get_client(&payload.charger_id).await?;
        client.cancel_outlet_reservation(payload).await
    }

    #[instrument]
    async fn change_charger_availability(
        &self,
        request: Request<ChangeChargerAvailabilityRequest>,
    ) -> Result<Response<ChangeChargerAvailabilityResponse>, Status> {
        let payload = request.into_inner();
        let mut client = self.get_client(&payload.charger_id).await?;
        client.change_charger_availability(payload).await
    }

    #[instrument]
    async fn change_evse_availability(
        &self,
        request: Request<ChangeEvseAvailabilityRequest>,
    ) -> Result<Response<ChangeEvseAvailabilityResponse>, Status> {
        let payload = request.into_inner();
        let mut client = self.get_client(&payload.charger_id).await?;
        client.change_evse_availability(payload).await
    }

    #[instrument]
    async fn change_connector_availability(
        &self,
        request: Request<ChangeConnectorAvailabilityRequest>,
    ) -> Result<Response<ChangeConnectorAvailabilityResponse>, Status> {
        let payload = request.into_inner();
        let mut client = self.get_client(&payload.charger_id).await?;
        client.change_connector_availability(payload).await
    }

    #[instrument]
    async fn change_ocpp1_6configuration_value(
        &self,
        request: Request<ChangeOcpp16configurationValueRequest>,
    ) -> Result<Response<ChangeOcpp16configurationValueResponse>, Status> {
        let payload = request.into_inner();
        let mut client = self.get_client(&payload.charger_id).await?;
        client.change_ocpp1_6configuration_value(payload).await
    }

    #[instrument]
    async fn clear_charger_cache(
        &self,
        request: Request<ClearChargerCacheRequest>,
    ) -> Result<Response<ClearChargerCacheResponse>, Status> {
        let payload = request.into_inner();
        let mut client = self.get_client(&payload.charger_id).await?;
        client.clear_charger_cache(payload).await
    }

    #[instrument]
    async fn start_transaction(
        &self,
        request: Request<StartTransactionRequest>,
    ) -> Result<Response<StartTransactionResponse>, Status> {
        let payload = request.into_inner();
        let mut client = self.get_client(&payload.charger_id).await?;
        client.start_transaction(payload).await
    }

    #[instrument]
    async fn stop_transaction(
        &self,
        request: Request<StopTransactionRequest>,
    ) -> Result<Response<StopTransactionResponse>, Status> {
        let payload = request.into_inner();
        let mut client = self.get_client(&payload.charger_id).await?;
        client.stop_transaction(payload).await
    }
}

impl From<(shared::ChargerData, ChargerConnectionInfo)> for Charger {
    fn from((charger, connection_info): (shared::ChargerData, ChargerConnectionInfo)) -> Self {
        Self {
            id: charger.id,
            serial_number: charger.serial_number,
            model: charger.model,
            vendor: charger.vendor,
            firmware_version: charger.firmware_version,
            iccid: charger.iccid,
            imsi: charger.imsi,
            ocpp1_6_configuration_values: charger
                .ocpp1_6configuration
                .map(|values| {
                    values
                        .iter()
                        .map(|(key, value)| Ocpp16configuration {
                            key: key.to_string(),
                            value: value.value.clone(),
                            readonly: value.read_only,
                        })
                        .collect()
                })
                .unwrap_or_default(),
            evses: charger
                .evses
                .into_iter()
                .map(|data| Evse {
                    id: data.id.to_string(),
                    ocpp_id: data.ocpp_evse_id,
                    connectors: data
                        .connectors
                        .into_iter()
                        .map(|connector| crate::ocpp_csms_server::Connector {
                            id: connector.id.to_string(),
                            ocpp_id: connector.ocpp_id,
                            r#type: crate::ocpp_csms_server::ConnectorType::from(
                                connector.connector_type,
                            )
                            .into(),
                            status: crate::ocpp_csms_server::ConnectorStatus::from(
                                connector.status.clone(),
                            )
                            .into(),
                        })
                        .collect(),
                })
                .collect(),
            is_online: connection_info.is_online,
            last_seen: connection_info.last_seen.to_rfc3339(),
            node_address: connection_info.node_address,
        }
    }
}

impl From<shared::ConnectorStatus> for crate::ocpp_csms_server::ConnectorStatus {
    fn from(status: shared::ConnectorStatus) -> Self {
        match status {
            shared::ConnectorStatus::Available => Self::Available,
            shared::ConnectorStatus::Occupied => Self::Occupied,
            shared::ConnectorStatus::Reserved => Self::Reserved,
            shared::ConnectorStatus::Unavailable => Self::Unavailable,
            shared::ConnectorStatus::Faulted => Self::Faulted,
        }
    }
}

impl From<shared::ConnectorType> for crate::ocpp_csms_server::ConnectorType {
    fn from(connector_type: shared::ConnectorType) -> Self {
        match connector_type {
            shared::ConnectorType::ConnectorCcs1 => Self::ConnectorCcs1,
            shared::ConnectorType::ConnectorCcs2 => Self::ConnectorCcs2,
            shared::ConnectorType::ConnectorG105 => Self::ConnectorG105,
            shared::ConnectorType::ConnectorTesla => Self::ConnectorTesla,
            shared::ConnectorType::ConnectorType1 => Self::ConnectorType1,
            shared::ConnectorType::ConnectorType2 => Self::ConnectorType2,
            shared::ConnectorType::Socket3091p16a => Self::Socket3091p16a,
            shared::ConnectorType::Socket3091p32a => Self::Socket3091p32a,
            shared::ConnectorType::Socket3093p16a => Self::Socket3093p16a,
            shared::ConnectorType::Socket3093p32a => Self::Socket3093p32a,
            shared::ConnectorType::SocketBs1361 => Self::SocketBs1361,
            shared::ConnectorType::SocketCee77 => Self::SocketCee77,
            shared::ConnectorType::SocketType2 => Self::SocketType2,
            shared::ConnectorType::SocketType3 => Self::SocketType3,
            shared::ConnectorType::Other1phMax16a => Self::Other1phMax16a,
            shared::ConnectorType::Other1phOver16a => Self::Other1phOver16a,
            shared::ConnectorType::Other3ph => Self::Other3ph,
            shared::ConnectorType::Pantograph => Self::Pantograph,
            shared::ConnectorType::WirelessInductive => Self::WirelessInductive,
            shared::ConnectorType::WirelessResonant => Self::WirelessResonant,
            shared::ConnectorType::Undetermined => Self::Undetermined,
            shared::ConnectorType::Unknown => Self::Unknown,
        }
    }
}
