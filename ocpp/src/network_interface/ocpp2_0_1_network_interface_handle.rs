use crate::network_interface::NetworkInterfaceHandle;

#[async_trait::async_trait]
pub trait Ocpp2_0_1NetworkInterfaceHandle: NetworkInterfaceHandle {}

#[cfg(test)]
mockall::mock! {
    pub Ocpp2_0_1NetworkInterfaceHandle {}
    #[async_trait::async_trait]
    impl Ocpp2_0_1NetworkInterfaceHandle for Ocpp2_0_1NetworkInterfaceHandle {
    }
    #[async_trait::async_trait]
    impl NetworkInterfaceHandle for Ocpp2_0_1NetworkInterfaceHandle {
        async fn disconnect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    }
}
