use crate::charger::ocpp1_6::update_metric::update_metric;
use chrono::{DateTime, Utc};
use ocpp_client::ocpp_1_6::OCPP1_6Error;
use rust_ocpp::v1_6::types::Phase;
use shared::ChargerData;

pub fn update_evse_ampere_from_metric_request(
    data: &mut ChargerData,
    connector_id: u32,
    timestamp: DateTime<Utc>,
    value: &str,
    phase: Option<Phase>,
) -> Result<(), OCPP1_6Error> {
    if let Some(phase) = phase {
        if let Some(evse) = data.evse_by_ocpp_id_mut(connector_id) {
            let metric = match phase {
                Phase::L1 => &mut evse.ampere_output.l1,
                Phase::L2 => &mut evse.ampere_output.l2,
                Phase::L3 => &mut evse.ampere_output.l3,
                Phase::L1N => &mut evse.ampere_output.l1,
                Phase::L2N => &mut evse.ampere_output.l2,
                Phase::L3N => &mut evse.ampere_output.l3,
                _ => return Ok(()),
            };
            update_metric(metric, value, timestamp)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rust_ocpp::v1_6::types::Phase;
    use shared::{ChargerData, EvseData, PhaseMetric};

    fn setup_charger_with_evse(connector_id: u32) -> ChargerData {
        let mut charger = ChargerData::default();
        let evse = EvseData {
            id: Default::default(),
            ocpp_evse_id: connector_id,
            connectors: vec![],
            watt_output: Default::default(),
            ampere_output: PhaseMetric::default(),
            voltage: Default::default(),
        };
        charger.evses.push(evse);
        charger
    }

    #[test]
    fn test_update_success_l1() {
        let mut charger = setup_charger_with_evse(1);
        let now = Utc::now();
        let res =
            update_evse_ampere_from_metric_request(&mut charger, 1, now, "12.5", Some(Phase::L1));
        assert!(res.is_ok());
        let evse = charger.evse_by_ocpp_id(1).unwrap();
        assert_eq!(evse.ampere_output.l1.value, 12.5);
        assert_eq!(evse.ampere_output.l1.measured_at, Some(now));
    }

    #[test]
    fn test_update_older_timestamp_no_change() {
        let mut charger = setup_charger_with_evse(1);
        let now = Utc::now();
        let earlier = now - chrono::Duration::seconds(10);
        // Set initial value
        update_evse_ampere_from_metric_request(&mut charger, 1, now, "10.0", Some(Phase::L1))
            .unwrap();
        // Try to update with older timestamp
        update_evse_ampere_from_metric_request(&mut charger, 1, earlier, "20.0", Some(Phase::L1))
            .unwrap();
        let evse = charger.evse_by_ocpp_id(1).unwrap();
        assert_eq!(evse.ampere_output.l1.value, 10.0);
    }

    #[test]
    fn test_update_invalid_value() {
        let mut charger = setup_charger_with_evse(1);
        let now = Utc::now();
        let res = update_evse_ampere_from_metric_request(
            &mut charger,
            1,
            now,
            "not_a_number",
            Some(Phase::L1),
        );
        assert!(res.is_err());
    }

    #[test]
    fn test_update_nonexistent_evse() {
        let mut charger = ChargerData::default();
        let now = Utc::now();
        let res =
            update_evse_ampere_from_metric_request(&mut charger, 99, now, "5.0", Some(Phase::L1));
        assert!(res.is_ok());
    }

    #[test]
    fn test_update_with_none_phase() {
        let mut charger = setup_charger_with_evse(1);
        let now = Utc::now();
        let res = update_evse_ampere_from_metric_request(&mut charger, 1, now, "5.0", None);
        assert!(res.is_ok());
        let evse = charger.evse_by_ocpp_id(1).unwrap();
        assert_eq!(evse.ampere_output.l1.value, 0.0);
    }
}
