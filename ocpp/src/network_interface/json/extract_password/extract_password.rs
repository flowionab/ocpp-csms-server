use crate::network_interface::json::extract_password::parse_authorization_header_value::parse_authorization_header_value;
use poem::Response;
use poem::http::HeaderMap;
use poem::http::header::AUTHORIZATION;
use tracing::info;

#[allow(clippy::result_large_err)]
pub fn extract_password(header_map: &HeaderMap) -> Result<Option<String>, Response> {
    match header_map.get(AUTHORIZATION) {
        None => {
            info!("No Authorization header provided");
            Ok(None)
        }
        Some(header) => {
            let value = parse_authorization_header_value(header)?;
            Ok(Some(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::http::HeaderMap;

    #[test]
    fn test_it_should_return_none_when_no_authorization_header() {
        let header_map = HeaderMap::new();
        let result = extract_password(&header_map);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_it_should_return_some_password_when_authorization_header_present() {
        let mut header_map = HeaderMap::new();
        header_map.insert(AUTHORIZATION, "Basic dXNlcjpwYXNz".parse().unwrap());
        let result = extract_password(&header_map);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("pass".to_string()));
    }
}
