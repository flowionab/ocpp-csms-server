use ocpp_client::ocpp_1_6::OCPP1_6Error;
use tonic::Status;

pub fn map_ocpp1_6_error_to_status(error: OCPP1_6Error) -> tonic::Status {
    match error {
        OCPP1_6Error::NotImplemented { description, .. } => Status::unimplemented(description),
        OCPP1_6Error::NotSupported { description, .. } => Status::unavailable(description),
        OCPP1_6Error::InternalError { description, .. } => Status::internal(description),
        OCPP1_6Error::ProtocolError { description, .. } => Status::internal(description),
        OCPP1_6Error::SecurityError { description, .. } => Status::internal(description),
        OCPP1_6Error::FormationViolation { description, .. } => Status::internal(description),
        OCPP1_6Error::PropertyConstraintViolation { description, .. } => {
            Status::internal(description)
        }
        OCPP1_6Error::OccurenceConstraintViolation { description, .. } => {
            Status::internal(description)
        }
        OCPP1_6Error::TypeConstraintViolation { description, .. } => Status::internal(description),
        OCPP1_6Error::GenericError { description, .. } => Status::unknown(description),
    }
}
