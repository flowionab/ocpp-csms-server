use poem::http::StatusCode;
use poem::Response;
use tracing::log::warn;

#[allow(clippy::result_large_err)]
pub fn split_and_get_password(value: &str) -> Result<String, Response> {
    let splits = value.split(":").collect::<Vec<_>>();
    match splits.get(1) {
        None => {
            warn!("Authorization header was provided, but the content was not correct formatted, it should have <username>:<password> layout");
            Err(Response::builder().status(StatusCode::BAD_REQUEST).body(
                "Authorization header was provided, but the content was not correct formatted, it should have <username>:<password> layout".to_string(),
            ))
        }
        Some(part) => Ok(part.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::http::StatusCode;

    #[test]
    fn test_valid_username_password() {
        let input = "user:secret";
        let result = split_and_get_password(input);
        assert_eq!(result.unwrap(), "secret");
    }

    #[test]
    fn test_missing_colon() {
        let input = "useronly";
        let result = split_and_get_password(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_empty_password() {
        let input = "user:";
        let result = split_and_get_password(input);
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_multiple_colons() {
        let input = "user:pass:extra";
        let result = split_and_get_password(input);
        assert_eq!(result.unwrap(), "pass");
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let result = split_and_get_password(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
    }
}
