use crate::network_interface::ProtocolHandle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, strum::Display, Eq, PartialEq)]
pub enum OcppProtocol {
    #[strum(to_string = "ocpp1.6")]
    Ocpp1_6,

    #[strum(to_string = "ocpp2.0.1")]
    Ocpp2_0_1,
}

impl TryFrom<&str> for OcppProtocol {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ocpp1.6" => Ok(OcppProtocol::Ocpp1_6),
            "ocpp2.0.1" => Ok(OcppProtocol::Ocpp2_0_1),
            _ => Err("Protocol not recognized".into()),
        }
    }
}

impl From<ProtocolHandle> for OcppProtocol {
    fn from(handle: ProtocolHandle) -> Self {
        match handle {
            ProtocolHandle::Ocpp1_6(_) => OcppProtocol::Ocpp1_6,
            ProtocolHandle::Ocpp2_0_1(_) => OcppProtocol::Ocpp2_0_1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network_interface::ocpp1_6_network_interface_handle::MockOcpp16NetworkInterfaceHandle;
    use crate::network_interface::ocpp2_0_1_network_interface_handle::MockOcpp2_0_1NetworkInterfaceHandle;
    use crate::network_interface::ProtocolHandle;
    use std::sync::Arc;

    #[test]
    fn test_try_from_str_valid_ocpp1_6() {
        let protocol = OcppProtocol::try_from("ocpp1.6").unwrap();
        assert_eq!(protocol, OcppProtocol::Ocpp1_6);
    }

    #[test]
    fn test_try_from_str_valid_ocpp2_0_1() {
        let protocol = OcppProtocol::try_from("ocpp2.0.1").unwrap();
        assert_eq!(protocol, OcppProtocol::Ocpp2_0_1);
    }

    #[test]
    fn test_try_from_str_invalid() {
        let err = OcppProtocol::try_from("invalid").unwrap_err();
        assert_eq!(err.to_string(), "Protocol not recognized");
    }

    #[test]
    fn test_from_protocol_handle_ocpp1_6() {
        let handle = ProtocolHandle::Ocpp1_6(Arc::new(MockOcpp16NetworkInterfaceHandle::new()));
        let proto = OcppProtocol::from(handle);
        assert_eq!(proto, OcppProtocol::Ocpp1_6);
    }

    #[test]
    fn test_from_protocol_handle_ocpp2_0_1() {
        let handle =
            ProtocolHandle::Ocpp2_0_1(Arc::new(MockOcpp2_0_1NetworkInterfaceHandle::new()));
        let proto = OcppProtocol::from(handle);
        assert_eq!(proto, OcppProtocol::Ocpp2_0_1);
    }
}
