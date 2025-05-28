pub fn select_protocol(header: &str) -> Option<&'static str> {
    for protocol in header.split(",") {
        if protocol == "ocpp1.6" {
            return Some("ocpp1.6");
        }
        if protocol == "ocpp2.0.1" {
            return Some("ocpp2.0.1");
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_ocpp16() {
        let header = "ocpp1.6";
        assert_eq!(select_protocol(header), Some("ocpp1.6"));
    }

    #[test]
    fn test_select_ocpp201() {
        let header = "ocpp2.0.1";
        assert_eq!(select_protocol(header), Some("ocpp2.0.1"));
    }

    #[test]
    fn test_select_none() {
        let header = "other,foo,bar";
        assert_eq!(select_protocol(header), None);
    }

    #[test]
    fn test_select_first_match() {
        let header = "ocpp2.0.1,ocpp1.6";
        assert_eq!(select_protocol(header), Some("ocpp2.0.1"));
    }

    #[test]
    fn test_select_with_spaces() {
        let header = "  ocpp1.6  , ocpp2.0.1 ";
        // The function does not trim, so this should return None
        assert_eq!(select_protocol(header), None);
    }

    #[test]
    fn test_select_multiple_protocols() {
        let header = "foo,ocpp1.6,bar";
        assert_eq!(select_protocol(header), Some("ocpp1.6"));
    }
}
