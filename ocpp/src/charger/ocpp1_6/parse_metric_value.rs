use ocpp_client::ocpp_1_6::OCPP1_6Error;
use tracing::error;

pub fn parse_metric_value<T: std::str::FromStr>(value: &str) -> Result<T, OCPP1_6Error> {
    let value = value.parse::<T>().map_err(|_e| {
        error!("Failed to parse metric value");
        OCPP1_6Error::new_formation_violation("Failed to parse metric value")
    })?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ocpp_client::ocpp_1_6::OCPP1_6Error;

    #[test]
    fn test_parse_metric_value_success_int() {
        let result: Result<i32, OCPP1_6Error> = parse_metric_value("42");
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_parse_metric_value_success_float() {
        let result: Result<f64, OCPP1_6Error> = parse_metric_value("3.14");
        assert!((result.unwrap() - 3.14).abs() < 1e-10);
    }

    #[test]
    fn test_parse_metric_value_failure() {
        let result: Result<i32, OCPP1_6Error> = parse_metric_value("not_a_number");
        assert!(result.is_err());
    }
}
