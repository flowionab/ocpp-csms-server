use crate::network_interface::{Ocpp2_0_1NetworkInterfaceHandle, Ocpp16NetworkInterfaceHandle};
use std::sync::Arc;

#[derive(Clone)]
pub enum ProtocolHandle {
    Ocpp1_6(Arc<dyn Ocpp16NetworkInterfaceHandle + Send + Sync>),
    Ocpp2_0_1(Arc<dyn Ocpp2_0_1NetworkInterfaceHandle + Send + Sync>),
}

impl ProtocolHandle {
    pub fn as_ocpp1_6(&self) -> Option<Arc<dyn Ocpp16NetworkInterfaceHandle + Send + Sync>> {
        if let ProtocolHandle::Ocpp1_6(handle) = self {
            Some(Arc::clone(handle))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn as_ocpp2_0_1(&self) -> Option<Arc<dyn Ocpp2_0_1NetworkInterfaceHandle + Send + Sync>> {
        if let ProtocolHandle::Ocpp2_0_1(handle) = self {
            Some(Arc::clone(handle))
        } else {
            None
        }
    }

    pub async fn disconnect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match self {
            ProtocolHandle::Ocpp1_6(handle) => handle.disconnect().await,
            ProtocolHandle::Ocpp2_0_1(handle) => handle.disconnect().await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network_interface::ocpp1_6_network_interface_handle::MockOcpp16NetworkInterfaceHandle;
    use crate::network_interface::ocpp2_0_1_network_interface_handle::MockOcpp2_0_1NetworkInterfaceHandle;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_as_ocpp1_6_returns_some() {
        let handle = Arc::new(MockOcpp16NetworkInterfaceHandle::new());
        let protocol = ProtocolHandle::Ocpp1_6(handle.clone());
        let result = protocol.as_ocpp1_6();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_as_ocpp1_6_returns_none() {
        let handle = Arc::new(MockOcpp2_0_1NetworkInterfaceHandle::new());
        let protocol = ProtocolHandle::Ocpp2_0_1(handle);
        let result = protocol.as_ocpp1_6();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_as_ocpp2_0_1_returns_some() {
        let handle = Arc::new(MockOcpp2_0_1NetworkInterfaceHandle::new());
        let protocol = ProtocolHandle::Ocpp2_0_1(handle.clone());
        let result = protocol.as_ocpp2_0_1();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_as_ocpp2_0_1_returns_none() {
        let handle = Arc::new(MockOcpp16NetworkInterfaceHandle::new());
        let protocol = ProtocolHandle::Ocpp1_6(handle);
        let result = protocol.as_ocpp2_0_1();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_disconnect_ocpp1_6() {
        let mut mock = MockOcpp16NetworkInterfaceHandle::new();
        mock.expect_disconnect().returning(|| Ok(()));
        let handle = Arc::new(mock);
        let protocol = ProtocolHandle::Ocpp1_6(handle);
        let result = protocol.disconnect().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_disconnect_ocpp2_0_1() {
        let mut mock = MockOcpp2_0_1NetworkInterfaceHandle::new();
        mock.expect_disconnect().returning(|| Ok(()));
        let handle = Arc::new(mock);
        let protocol = ProtocolHandle::Ocpp2_0_1(handle);
        let result = protocol.disconnect().await;
        assert!(result.is_ok());
    }
}
