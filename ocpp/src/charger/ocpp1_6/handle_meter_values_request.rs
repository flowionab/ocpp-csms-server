use crate::charger::ocpp1_6::update_evse_ampere_from_metric_request::update_evse_ampere_from_metric_request;
use crate::charger::ocpp1_6::update_evse_voltage_from_metric_request::update_evse_voltage_from_metric_request;
use ocpp_client::ocpp_1_6::OCPP1_6Error;
use rust_ocpp::v1_6::messages::meter_values::MeterValuesRequest;
use rust_ocpp::v1_6::types::Measurand;
use shared::ChargerData;

pub fn update_charger_from_meter_values_request(
    data: &mut ChargerData,
    request: &MeterValuesRequest,
) -> Result<(), OCPP1_6Error> {
    for meter_value in &request.meter_value {
        for sampled_value in &meter_value.sampled_value {
            match sampled_value.measurand.clone().unwrap_or_default() {
                Measurand::CurrentExport => {}
                Measurand::CurrentImport => {
                    update_evse_ampere_from_metric_request(
                        data,
                        request.connector_id,
                        meter_value.timestamp,
                        &sampled_value.value,
                        sampled_value.phase.clone(),
                    )?;
                }
                Measurand::CurrentOffered => {}
                Measurand::EnergyActiveExportRegister => {}
                Measurand::EnergyActiveImportRegister => {}
                Measurand::EnergyReactiveExportRegister => {}
                Measurand::EnergyReactiveImportRegister => {}
                Measurand::EnergyActiveExportInterval => {}
                Measurand::EnergyActiveImportInterval => {}
                Measurand::EnergyReactiveExportInterval => {}
                Measurand::EnergyReactiveImportInterval => {}
                Measurand::Frequency => {}
                Measurand::PowerActiveExport => {}
                Measurand::PowerActiveImport => {}
                Measurand::PowerFactor => {}
                Measurand::PowerOffered => {}
                Measurand::PowerReactiveExport => {}
                Measurand::PowerReactiveImport => {}
                Measurand::Rpm => {}
                Measurand::SoC => {}
                Measurand::Temperature => {}
                Measurand::Voltage => {
                    update_evse_voltage_from_metric_request(
                        data,
                        request.connector_id,
                        meter_value.timestamp,
                        &sampled_value.value,
                        sampled_value.phase.clone(),
                    )?;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rust_ocpp::v1_6::messages::meter_values::MeterValuesRequest;
    use rust_ocpp::v1_6::types::{Measurand, MeterValue, Phase, SampledValue};
    use shared::{ChargerData, Config, EvseData, PhaseMetric};

    fn setup_charger_with_evse(connector_id: u32) -> ChargerData {
        let mut charger = ChargerData::new("test", &Config::default());
        let evse = EvseData {
            id: Default::default(),
            ocpp_evse_id: connector_id,
            connectors: vec![],
            watt_output: Default::default(),
            ampere_output: PhaseMetric::default(),
            voltage: PhaseMetric::default(),
        };
        charger.evses.push(evse);
        charger
    }

    #[test]
    fn test_handle_meter_values_current_import() {
        let mut charger = setup_charger_with_evse(1);
        let now = Utc::now();
        let request = MeterValuesRequest {
            connector_id: 1,
            transaction_id: None,
            meter_value: vec![MeterValue {
                timestamp: now,
                sampled_value: vec![SampledValue {
                    value: "12.5".to_string(),
                    measurand: Some(Measurand::CurrentImport),
                    phase: Some(Phase::L1),
                    ..Default::default()
                }],
            }],
        };
        let res = update_charger_from_meter_values_request(&mut charger, &request);
        assert!(res.is_ok());
        let evse = charger.evse_by_ocpp_id(1).unwrap();
        assert_eq!(evse.ampere_output.l1.value, 12.5);
    }

    #[test]
    fn test_handle_meter_values_voltage() {
        let mut charger = setup_charger_with_evse(1);
        let now = Utc::now();
        let request = MeterValuesRequest {
            connector_id: 1,
            transaction_id: None,
            meter_value: vec![MeterValue {
                timestamp: now,
                sampled_value: vec![SampledValue {
                    value: "230.0".to_string(),
                    measurand: Some(Measurand::Voltage),
                    phase: Some(Phase::L2),
                    ..Default::default()
                }],
            }],
        };
        let res = update_charger_from_meter_values_request(&mut charger, &request);
        assert!(res.is_ok());
        let evse = charger.evse_by_ocpp_id(1).unwrap();
        assert_eq!(evse.voltage.l2.value, 230.0);
    }

    #[test]
    fn test_handle_meter_values_unsupported_measurand() {
        let mut charger = setup_charger_with_evse(1);
        let now = Utc::now();
        let request = MeterValuesRequest {
            connector_id: 1,
            transaction_id: None,
            meter_value: vec![MeterValue {
                timestamp: now,
                sampled_value: vec![SampledValue {
                    value: "999.0".to_string(),
                    measurand: Some(Measurand::PowerActiveImport),
                    phase: None,
                    ..Default::default()
                }],
            }],
        };
        // Should not update anything, but also not error
        let res = update_charger_from_meter_values_request(&mut charger, &request);
        assert!(res.is_ok());
        let evse = charger.evse_by_ocpp_id(1).unwrap();
        assert_eq!(evse.ampere_output.l1.value, 0.0);
        assert_eq!(evse.voltage.l1.value, 0.0);
    }

    #[test]
    fn test_handle_meter_values_invalid_value() {
        let mut charger = setup_charger_with_evse(1);
        let now = Utc::now();
        let request = MeterValuesRequest {
            connector_id: 1,
            transaction_id: None,
            meter_value: vec![MeterValue {
                timestamp: now,
                sampled_value: vec![SampledValue {
                    value: "not_a_number".to_string(),
                    measurand: Some(Measurand::CurrentImport),
                    phase: Some(Phase::L1),
                    ..Default::default()
                }],
            }],
        };
        let res = update_charger_from_meter_values_request(&mut charger, &request);
        assert!(res.is_err());
    }
}
