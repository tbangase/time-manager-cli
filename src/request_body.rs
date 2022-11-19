use derive_getters::Getters;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Getters, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AppScriptRequest {
    function: String,
    #[builder(default)]
    parameters: Vec<String>,
    #[builder(default)]
    session_state: Option<String>,
    #[builder(default)]
    dev_mode: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn build_and_serialize_test() {
        let body = AppScriptRequest::builder()
            .function("test".to_string())
            .parameters(vec!["test".to_string()])
            .build();
        let json = serde_json::to_string(&body).unwrap();

        tracing::debug!("{:#?}", json);

        assert_eq!(
            json,
            r#"{"function":"test","parameters":["test"],"sessionState":null,"devMode":false}"#
        );
    }
}
