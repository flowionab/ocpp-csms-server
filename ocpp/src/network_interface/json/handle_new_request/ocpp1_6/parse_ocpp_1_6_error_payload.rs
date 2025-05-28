use ocpp_client::ocpp_1_6::OCPP1_6Error;
use serde_json::Value;

pub fn parse_ocpp_1_6_error_payload(
    payload: &str,
) -> Result<(String, OCPP1_6Error), Box<dyn std::error::Error + Send + Sync>> {
    let (_, message_id, code, description, details): (i64, String, String, String, Value) =
        serde_json::from_str(payload)?;

    let err = match code.as_str() {
        "NotImplemented" => OCPP1_6Error::NotImplemented {
            description,
            details,
        },
        "NotSupported" => OCPP1_6Error::NotSupported {
            description,
            details,
        },
        "InternalError" => OCPP1_6Error::InternalError {
            description,
            details,
        },
        "ProtocolError" => OCPP1_6Error::ProtocolError {
            description,
            details,
        },
        "SecurityError" => OCPP1_6Error::SecurityError {
            description,
            details,
        },
        "FormationViolation" => OCPP1_6Error::FormationViolation {
            description,
            details,
        },
        "PropertyConstraintViolation" => OCPP1_6Error::PropertyConstraintViolation {
            description,
            details,
        },
        "OccurenceConstraintViolation" => OCPP1_6Error::OccurenceConstraintViolation {
            description,
            details,
        },
        "TypeConstraintViolation" => OCPP1_6Error::TypeConstraintViolation {
            description,
            details,
        },
        "GenericError" => OCPP1_6Error::GenericError {
            description,
            details,
        },
        _ => OCPP1_6Error::GenericError {
            description,
            details,
        },
    };
    Ok((message_id, err))
}
