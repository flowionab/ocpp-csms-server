use crate::charger::ocpp1_6::update_metric::update_metric;
use chrono::{DateTime, Utc};
use ocpp_client::ocpp_1_6::OCPP1_6Error;
use rust_ocpp::v1_6::types::Phase;
use shared::data::ChargerData;

pub fn update_evse_voltage_from_metric_request(
    data: &mut ChargerData,
    connector_id: u32,
    timestamp: DateTime<Utc>,
    value: &str,
    phase: Option<Phase>,
) -> Result<(), OCPP1_6Error> {
    if let Some(phase) = phase {
        if let Some(evse) = data.evse_by_ocpp_id_mut(connector_id) {
            let metric = match phase {
                Phase::L1 => &mut evse.voltage.l1,
                Phase::L2 => &mut evse.voltage.l2,
                Phase::L3 => &mut evse.voltage.l3,
                Phase::L1N => &mut evse.voltage.l1,
                Phase::L2N => &mut evse.voltage.l2,
                Phase::L3N => &mut evse.voltage.l3,
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
    use shared::data::{ChargerData, EvseData, PhaseMetric};

    fn setup_charger_with_evse(connector_id: u32) -> ChargerData {
        let mut charger = ChargerData::default();
        let evse = EvseData {
            id: Default::default(),
            ocpp_evse_id: connector_id,
            connectors: vec![],
            watt_output: Default::default(),
            ampere_output: Default::default(),
            voltage: PhaseMetric::default(),
        };
        charger.evses.push(evse);
        charger
    }

    #[test]
    fn test_update_voltage_l1() {
        let mut charger = setup_charger_with_evse(1);
        let now = Utc::now();
        let res =
            update_evse_voltage_from_metric_request(&mut charger, 1, now, "230.0", Some(Phase::L1));
        assert!(res.is_ok());
        let evse = charger.evse_by_ocpp_id(1).unwrap();
        assert_eq!(evse.voltage.l1.value, 230.0);
        assert_eq!(evse.voltage.l1.measured_at, Some(now));
    }

    #[test]
    fn test_update_voltage_l2n() {
        let mut charger = setup_charger_with_evse(1);
        let now = Utc::now();
        let res = update_evse_voltage_from_metric_request(
            &mut charger,
            1,
            now,
            "231.0",
            Some(Phase::L2N),
        );
        assert!(res.is_ok());
        let evse = charger.evse_by_ocpp_id(1).unwrap();
        assert_eq!(evse.voltage.l2.value, 231.0);
    }

    #[test]
    fn test_update_voltage_invalid_value() {
        let mut charger = setup_charger_with_evse(1);
        let now = Utc::now();
        let res = update_evse_voltage_from_metric_request(
            &mut charger,
            1,
            now,
            "not_a_number",
            Some(Phase::L1),
        );
        assert!(res.is_err());
    }

    #[test]
    fn test_update_voltage_nonexistent_evse() {
        let mut charger = ChargerData::default();
        let now = Utc::now();
        let res = update_evse_voltage_from_metric_request(
            &mut charger,
            99,
            now,
            "220.0",
            Some(Phase::L1),
        );
        assert!(res.is_ok());
    }

    #[test]
    fn test_update_voltage_none_phase() {
        let mut charger = setup_charger_with_evse(1);
        let now = Utc::now();
        let res = update_evse_voltage_from_metric_request(&mut charger, 1, now, "220.0", None);
        assert!(res.is_ok());
        let evse = charger.evse_by_ocpp_id(1).unwrap();
        // Should not update any phase
        assert_eq!(evse.voltage.l1.value, 0.0);
        assert_eq!(evse.voltage.l2.value, 0.0);
        assert_eq!(evse.voltage.l3.value, 0.0);
    }
}
