use crate::ocpp_csms_server;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RebootType {
    #[default]
    Soft,
    Hard,
}

impl From<RebootType> for ocpp_csms_server::reboot_charger_request::RebootType {
    fn from(value: RebootType) -> Self {
        match value {
            RebootType::Soft => Self::Soft,
            RebootType::Hard => Self::Hard,
        }
    }
}
