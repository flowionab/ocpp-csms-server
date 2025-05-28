use crate::charger::charger_model::ChargerModel;
use crate::charger::Charger;
use crate::network_interface::json::AuthenticationHandler;
use poem::http::StatusCode;
use poem::Response;
use tracing::{error, warn};

#[async_trait::async_trait]
impl AuthenticationHandler for Charger {
    async fn authenticate_with_password(
        &mut self,
        password: &Option<String>,
    ) -> Result<(), Response> {
        self.password = password.clone();

        if let Some(ChargerModel::Easee(_)) = self.model() {
            return match &self.easee_master_password {
                Some(master_password) => {
                    if password.as_ref() == Some(master_password) {
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
                None => {
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
                    warn!(charger_id = self.id.to_string(), "The charger does have existing credentials, but it has not been onboarded yet to our system, ignoring the credentials for now...")
                }
                Ok(())
            }
        }
    }
}
