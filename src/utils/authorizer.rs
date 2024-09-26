use axum::http::HeaderMap;
use shuttle_runtime::SecretStore;

/// Dummy authorizer, checks for a valid token auth header, which is a static string
pub async fn authorizer(secret_store: &SecretStore, header_map: HeaderMap) -> bool {
    // Extract auth headers and validate them
    let valid_header = format!(
        "Bearer {}",
        secret_store
            .get("DUMMY_TOKEN")
            .expect("DUMMY_TOKEN variable not set!")
    );
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
    use shuttle_common::Secret;
    use std::collections::BTreeMap;
    use std::env;

    #[tokio::test]
    async fn test_authorizer() {
        // Loading secrets with dotenvy to get the dummy token for testing
        dotenvy::from_filename("Secrets.test.toml").expect("Failed to load secrets.toml for unit tests");
        let dummy_token = env::var("DUMMY_TOKEN").expect("couldn't load dummy token variable");

        // Build dummy secrets store for testing
        let mut secrets_tree: BTreeMap<String, Secret<String>> = BTreeMap::new();
        secrets_tree.insert("DUMMY_TOKEN".to_string(), Secret::new(dummy_token.clone()));
        let secrets = SecretStore::new(secrets_tree);

        // Test with valid token
        let mut header_map = HeaderMap::new();
        header_map.insert(
            "Authorization",
            HeaderValue::from_str(format!("Bearer {}", dummy_token).as_str()).unwrap(),
        );
        assert_eq!(authorizer(&secrets, header_map).await, true);

        // Test with invalid token
        let mut header_map = HeaderMap::new();
        header_map.insert(
            "Authorization",
            HeaderValue::from_str("Bearer test_dummy2").unwrap(),
        );
        assert_eq!(authorizer(&secrets, header_map).await, false);

        // Test with no token
        let header_map = HeaderMap::new();
        assert_eq!(authorizer(&secrets, header_map).await, false);
    }
}
