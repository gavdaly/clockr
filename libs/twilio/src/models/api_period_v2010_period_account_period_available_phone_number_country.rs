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
pub struct ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountry {
    /// The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country.
    #[serde(
        rename = "country_code",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub country_code: Option<Option<String>>,
    /// The name of the country.
    #[serde(
        rename = "country",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub country: Option<Option<String>>,
    /// The URI of the Country resource, relative to `https://api.twilio.com`.
    #[serde(
        rename = "uri",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub uri: Option<Option<String>>,
    /// Whether all phone numbers available in the country are new to the Twilio platform. `true` if they are and `false` if all numbers are not in the Twilio Phone Number Beta program.
    #[serde(
        rename = "beta",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub beta: Option<Option<bool>>,
    /// A list of related AvailablePhoneNumber resources identified by their URIs relative to `https://api.twilio.com`.
    #[serde(
        rename = "subresource_uris",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub subresource_uris: Option<Option<serde_json::Value>>,
}

impl Default for ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountry {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountry {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountry {
        ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountry {
            country_code: None,
            country: None,
            uri: None,
            beta: None,
            subresource_uris: None,
        }
    }
}
