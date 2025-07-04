use crate::ocpp_csms_server;
use crate::types::{Charger, ChargerSummary, RebootType, RfidScanSession, Transaction};
pub use ocpp_csms_server::api_client::ApiClient;
use tonic::transport::Channel;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct OcppApiClient {
    client: ApiClient<Channel>,
}

impl OcppApiClient {
    /// Creates a new instance of the OcppApiClient.
    pub async fn connect(
        url: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let client = ApiClient::connect(url.to_string()).await?;
        Ok(Self { client })
    }
    pub async fn create_charger(
        &self,
        charger_id: &str,
    ) -> Result<Option<Charger>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut client = self.client.clone();
        let request = ocpp_csms_server::CreateChargerRequest {
            charger_id: charger_id.to_string(),
        };

        let response = client.create_charger(request).await?;

        response
            .into_inner()
            .charger
            .map(Charger::try_from)
            .transpose()
    }

    /// Retrieves a charger by its ID.
    pub async fn get_charger(
        &self,
        charger_id: &str,
    ) -> Result<Option<Charger>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut client = self.client.clone();
        let request = ocpp_csms_server::GetChargerRequest {
            charger_id: charger_id.to_string(),
        };

        let response = client.get_charger(request).await?;

        response
            .into_inner()
            .charger
            .map(Charger::try_from)
            .transpose()
    }

    pub async fn get_chargers(
        &self,
        page_size: i64,
        page: i64,
    ) -> Result<Vec<ChargerSummary>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut client = self.client.clone();
        let request = ocpp_csms_server::GetChargersRequest { page_size, page };

        let response = client.get_chargers(request).await?;

        Ok(response
            .into_inner()
            .chargers
            .into_iter()
            .map(ChargerSummary::from)
            .collect())
    }

    pub async fn reboot_charger(
        &self,
        charger_id: &str,
        reboot_type: RebootType,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut client = self.client.clone();
        let request = ocpp_csms_server::RebootChargerRequest {
            charger_id: charger_id.to_string(),
            reboot_type: ocpp_csms_server::reboot_charger_request::RebootType::from(reboot_type)
                .into(),
        };

        let _ = client.reboot_charger(request).await?;

        Ok(())
    }

    pub async fn change_charger_availability(
        &self,
        charger_id: &str,
        operative: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut client = self.client.clone();
        let request = ocpp_csms_server::ChangeChargerAvailabilityRequest {
            charger_id: charger_id.to_string(),
            operative,
        };

        let _ = client.change_charger_availability(request).await?;

        Ok(())
    }

    pub async fn change_evse_availability(
        &self,
        charger_id: &str,
        evse_id: &str,
        operative: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut client = self.client.clone();
        let request = ocpp_csms_server::ChangeEvseAvailabilityRequest {
            charger_id: charger_id.to_string(),
            evse_id: evse_id.to_string(),
            operative,
        };

        let _ = client.change_evse_availability(request).await?;

        Ok(())
    }

    pub async fn change_connector_availability(
        &self,
        charger_id: &str,
        evse_id: &str,
        connector_id: &str,
        operative: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut client = self.client.clone();
        let request = ocpp_csms_server::ChangeConnectorAvailabilityRequest {
            charger_id: charger_id.to_string(),
            evse_id: evse_id.to_string(),
            connector_id: connector_id.to_string(),
            operative,
        };

        let _ = client.change_connector_availability(request).await?;

        Ok(())
    }

    pub async fn clear_charger_cache(
        &self,
        charger_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut client = self.client.clone();
        let request = ocpp_csms_server::ClearChargerCacheRequest {
            charger_id: charger_id.to_string(),
        };

        let _ = client.clear_charger_cache(request).await?;

        Ok(())
    }

    pub async fn start_transaction(
        &self,
        charger_id: &str,
        evse_id: &str,
    ) -> Result<Transaction, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut client = self.client.clone();
        let request = ocpp_csms_server::StartTransactionRequest {
            charger_id: charger_id.to_string(),
            evse_id: evse_id.to_string(),
        };

        let response = client.start_transaction(request).await?;

        match response.into_inner().transaction {
            Some(transaction) => Ok(Transaction::try_from(transaction)?),
            None => Err("Missing transaction".into()),
        }
    }

    pub async fn stop_transaction(
        &self,
        charger_id: &str,
        transaction_id: &str,
    ) -> Result<Transaction, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut client = self.client.clone();
        let request = ocpp_csms_server::StopTransactionRequest {
            charger_id: charger_id.to_string(),
            transaction_id: transaction_id.to_string(),
        };

        let response = client.stop_transaction(request).await?;

        match response.into_inner().transaction {
            Some(transaction) => Ok(Transaction::try_from(transaction)?),
            None => Err("Missing transaction".into()),
        }
    }

    pub async fn get_ongoing_transaction(
        &self,
        charger_id: &str,
        evse_id: Uuid,
    ) -> Result<Option<Transaction>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut client = self.client.clone();
        let request = ocpp_csms_server::GetOngoingTransactionRequest {
            charger_id: charger_id.to_string(),
            evse_id: evse_id.to_string(),
        };

        let response = client.get_ongoing_transaction(request).await?;

        response
            .into_inner()
            .transaction
            .map(Transaction::try_from)
            .transpose()
    }

    pub async fn create_rfid_scan_session(
        &self,
        charger_id: &str,
    ) -> Result<RfidScanSession, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut client = self.client.clone();
        let request = ocpp_csms_server::CreateRfidScanSessionRequest {
            charger_id: charger_id.to_string(),
        };

        let response = client.create_rfid_scan_session(request).await?;

        response
            .into_inner()
            .session
            .map(RfidScanSession::try_from)
            .ok_or_else(|| {
                Box::<dyn std::error::Error + Send + Sync + 'static>::from(
                    "Missing RfidScanSession",
                )
            })?
    }

    pub async fn get_rfid_scan_session(
        &self,
        session_id: Uuid,
    ) -> Result<Option<RfidScanSession>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut client = self.client.clone();
        let request = ocpp_csms_server::GetRfidScanSessionRequest {
            session_id: session_id.to_string(),
        };

        let response = client.get_rfid_scan_session(request).await?;

        response
            .into_inner()
            .session
            .map(RfidScanSession::try_from)
            .transpose()
    }
}
