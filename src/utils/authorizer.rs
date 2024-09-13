use axum::http::HeaderMap;

/// Dummy authorizer, checks for a valid token auth header, which is a static string
pub async fn authorizer(header_map: HeaderMap) -> bool {
    // Extract auth headers and validate them
    let valid_header = "Bearer iamasupersecrettoken".to_string();
    let auth_header = header_map.get("Authorization");

    // check if header is present and matches our token
    match auth_header {
        Some(header) => {
            let header = header.to_str().unwrap();
            if header != valid_header {
                return false;
            }
            true
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_authorizer() {
        // Test with valid token
        let mut header_map = HeaderMap::new();
        header_map.insert(
            "Authorization",
            HeaderValue::from_str("Bearer iamasupersecrettoken").unwrap(),
        );
        assert_eq!(authorizer(header_map).await, true);

        // Test with invalid token
        let mut header_map = HeaderMap::new();
        header_map.insert(
            "Authorization",
            HeaderValue::from_str("Bearer iamasupersecrettoken2").unwrap(),
        );
        assert_eq!(authorizer(header_map).await, false);

        // Test with no token
        let header_map = HeaderMap::new();
        assert_eq!(authorizer(header_map).await, false);
    }
}
