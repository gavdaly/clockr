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
pub struct ApiPeriodV2010PeriodAccountPeriodSigningKey {
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    #[serde(rename = "friendly_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<Option<String>>,
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
}

impl Default for ApiPeriodV2010PeriodAccountPeriodSigningKey {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiPeriodV2010PeriodAccountPeriodSigningKey {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodSigningKey {
        ApiPeriodV2010PeriodAccountPeriodSigningKey {
            sid: None,
            friendly_name: None,
            date_created: None,
            date_updated: None,
        }
    }
}


