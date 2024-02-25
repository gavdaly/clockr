/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.51.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiPeriodV2010PeriodAccountPeriodCallPeriodCallEvent {
    /// Contains a dictionary representing the request of the call.
    #[serde(
        rename = "request",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub request: Option<Option<serde_json::Value>>,
    /// Contains a dictionary representing the call response, including a list of the call events.
    #[serde(
        rename = "response",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub response: Option<Option<serde_json::Value>>,
}

impl Default for ApiPeriodV2010PeriodAccountPeriodCallPeriodCallEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiPeriodV2010PeriodAccountPeriodCallPeriodCallEvent {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodCallPeriodCallEvent {
        ApiPeriodV2010PeriodAccountPeriodCallPeriodCallEvent {
            request: None,
            response: None,
        }
    }
}
