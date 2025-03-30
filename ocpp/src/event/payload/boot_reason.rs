#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default, Copy)]
pub enum BootReason {
    ApplicationReset,
    FirmwareUpdate,
    LocalReset,
    #[default]
    PowerUp,
    RemoteReset,
    ScheduledReset,
    Triggered,
    Unknown,
    Watchdog,
}
