use ocpp_client::ocpp_2_0_1::OCPP2_0_1Error;
use serde_json::Value;

pub fn parse_ocpp_2_0_1_error_payload(
    payload: &str,
) -> Result<(String, OCPP2_0_1Error), Box<dyn std::error::Error + Send + Sync>> {
    let (_, message_id, code, description, details): (i64, String, String, String, Value) =
        serde_json::from_str(payload)?;

    let err = match code.as_str() {
        "NotImplemented" => OCPP2_0_1Error::NotImplemented {
            description,
            details,
        },
        "NotSupported" => OCPP2_0_1Error::NotSupported {
            description,
            details,
        },
        "InternalError" => OCPP2_0_1Error::InternalError {
            description,
            details,
        },
        "ProtocolError" => OCPP2_0_1Error::ProtocolError {
            description,
            details,
        },
        "SecurityError" => OCPP2_0_1Error::SecurityError {
            description,
            details,
        },
        "FormatViolation" => OCPP2_0_1Error::FormatViolation {
            description,
            details,
        },
        "PropertyConstraintViolation" => OCPP2_0_1Error::PropertyConstraintViolation {
            description,
            details,
        },
        "OccurrenceConstraintViolation" => OCPP2_0_1Error::OccurrenceConstraintViolation {
            description,
            details,
        },
        "TypeConstraintViolation" => OCPP2_0_1Error::TypeConstraintViolation {
            description,
            details,
        },
        "GenericError" => OCPP2_0_1Error::GenericError {
            description,
            details,
        },
        _ => OCPP2_0_1Error::GenericError {
            description,
            details,
        },
    };
    Ok((message_id, err))
}
