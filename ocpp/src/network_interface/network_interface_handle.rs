#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait NetworkInterfaceHandle {
    /// Disconnect from the network interface.
    async fn disconnect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
