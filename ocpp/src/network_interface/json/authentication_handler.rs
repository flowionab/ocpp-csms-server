use poem::Response;

#[async_trait::async_trait]
pub trait AuthenticationHandler {
    async fn authenticate_with_password(
        &mut self,
        password: &Option<String>,
    ) -> Result<(), Response>;
}
