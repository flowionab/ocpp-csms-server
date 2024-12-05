use crate::charger::ChargerPool;
use crate::ocpp_csms_server::ocpp_server::Ocpp;
use crate::ocpp_csms_server::{StartTransactionRequest, StartTransactionResponse};
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
                lock.start_transaction().await?;
                Ok(Response::new(StartTransactionResponse {}))
            }
            None => Err(Status::not_found(
                "A charger with this id is not connected to this instance",
            )),
        }
    }
}
