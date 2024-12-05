use crate::ocpp_csms_server::api_server::Api;
use crate::ocpp_csms_server::{Charger, GetChargerRequest, GetChargerResponse};
use shared::DataStore;
use tonic::{Request, Response, Status};
use tracing::error;

pub struct ApiService {
    data_store: Box<dyn DataStore>,
}

impl ApiService {
    pub fn new(data_store: Box<dyn DataStore>) -> Self {
        Self { data_store }
    }
}

#[tonic::async_trait]
impl Api for ApiService {
    async fn get_charger(
        &self,
        request: Request<GetChargerRequest>,
    ) -> Result<Response<GetChargerResponse>, Status> {
        let payload = request.into_inner();
        let charger = self
            .data_store
            .get_charger_data_by_id(&payload.charger_id)
            .await
            .map_err(|error| {
                error!(error_message = error.to_string(), "could not get charger");
                Status::internal("Could not get charger")
            })?;

        Ok(Response::new(GetChargerResponse {
            charger: charger.map(|charger| Charger {
                id: charger.id,
                serial_number: charger.serial_number,
            }),
        }))
    }
}
