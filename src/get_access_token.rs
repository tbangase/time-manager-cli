use crate::{error_trace, AUTH_URL, CLIENT_SECRET, OAUTH_CLIENT_ID, REFRESH_TOKEN};
use anyhow::Context;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

pub async fn get_access_token() -> anyhow::Result<String> {
    let payload = Payload::builder()
        .client_id(OAUTH_CLIENT_ID)
        .client_secret(CLIENT_SECRET)
        .refresh_token(REFRESH_TOKEN)
        .grant_type("refresh_token")
        .build();

    tracing::debug!("Payload: {:#?}", payload);

    let request = reqwest::Client::new()
        .post(AUTH_URL)
        .body(serde_json::to_string(&payload)?);

    tracing::debug!("Request: {:#?}", request);

    let response = request.send().await?;

    tracing::debug!("Response: {:#?}", response);

    let res_str = format!("{:?}", response);

    let response_json = response
        .json::<Response>()
        .await
        .with_context(|| error_trace!("Response Json deserialize error:\n {}", res_str))?;

    Ok(response_json.access_token().clone())
}

#[derive(Debug, Deserialize, Getters)]
struct Response {
    access_token: String,
}

#[derive(Debug, TypedBuilder, Serialize)]
struct Payload<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    refresh_token: &'a str,
    grant_type: &'a str,
}

#[cfg(test)]
mod get_access_token_tests {
    use super::*;

    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn success_test() {
        let res = get_access_token().await;

        if let Err(ref e) = res {
            tracing::error!("{:?}", e);
        }

        assert!(res.is_ok());
    }
}
