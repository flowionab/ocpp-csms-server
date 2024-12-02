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
