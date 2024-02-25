/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.51.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

/// ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberLocalCapabilities : The set of Boolean properties that indicate whether a phone number can receive calls or messages.  Capabilities are: `Voice`, `SMS`, and `MMS` and each capability can be: `true` or `false`.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberLocalCapabilities {
    #[serde(rename = "mms", skip_serializing_if = "Option::is_none")]
    pub mms: Option<bool>,
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none")]
    pub sms: Option<bool>,
    #[serde(rename = "voice", skip_serializing_if = "Option::is_none")]
    pub voice: Option<bool>,
    #[serde(rename = "fax", skip_serializing_if = "Option::is_none")]
    pub fax: Option<bool>,
}

impl Default for ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberLocalCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberLocalCapabilities {
    /// The set of Boolean properties that indicate whether a phone number can receive calls or messages.  Capabilities are: `Voice`, `SMS`, and `MMS` and each capability can be: `true` or `false`.
    pub fn new() -> ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberLocalCapabilities
    {
        ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberLocalCapabilities {
            mms: None,
            sms: None,
            voice: None,
            fax: None,
        }
    }
}
