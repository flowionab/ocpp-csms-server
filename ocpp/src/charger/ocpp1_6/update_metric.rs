use crate::charger::ocpp1_6::parse_metric_value::parse_metric_value;
use chrono::{DateTime, Utc};
use ocpp_client::ocpp_1_6::OCPP1_6Error;
use shared::data::Metric;

pub fn update_metric(
    metric: &mut Metric<f32>,
    value: &str,
    timestamp: DateTime<Utc>,
) -> Result<(), OCPP1_6Error> {
    if metric.measured_at.is_none() || metric.measured_at.unwrap() < timestamp {
        metric.value = parse_metric_value::<f32>(value)?;
        metric.measured_at = Some(timestamp);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use shared::data::Metric;

    fn metric_with_value_and_time(val: f32, ts: i64) -> Metric<f32> {
        Metric {
            value: val,
            measured_at: Some(Utc.timestamp_opt(ts, 0).unwrap()),
            ..Default::default()
        }
    }

    #[test]
    fn test_update_metric_newer_timestamp() {
        let mut metric = metric_with_value_and_time(1.0, 1000);
        let new_time = Utc.timestamp_opt(2000, 0).unwrap();
        let res = update_metric(&mut metric, "2.5", new_time);
        assert!(res.is_ok());
        assert_eq!(metric.value, 2.5);
        assert_eq!(metric.measured_at, Some(new_time));
    }

    #[test]
    fn test_update_metric_older_timestamp() {
        let mut metric = metric_with_value_and_time(1.0, 2000);
        let old_time = Utc.timestamp_opt(1000, 0).unwrap();
        let res = update_metric(&mut metric, "3.5", old_time);
        assert!(res.is_ok());
        assert_eq!(metric.value, 1.0);
        assert_eq!(
            metric.measured_at,
            Some(Utc.timestamp_opt(2000, 0).unwrap())
        );
    }

    #[test]
    fn test_update_metric_none_timestamp() {
        let mut metric = Metric::<f32> {
            value: 0.0,
            measured_at: None,
            ..Default::default()
        };
        let ts = Utc.timestamp_opt(1234, 0).unwrap();
        let res = update_metric(&mut metric, "4.2", ts);
        assert!(res.is_ok());
        assert_eq!(metric.value, 4.2);
        assert_eq!(metric.measured_at, Some(ts));
    }

    #[test]
    fn test_update_metric_invalid_value() {
        let mut metric = Metric::<f32> {
            value: 0.0,
            measured_at: None,
            ..Default::default()
        };
        let ts = Utc::now();
        let res = update_metric(&mut metric, "not_a_number", ts);
        assert!(res.is_err());
    }
}
