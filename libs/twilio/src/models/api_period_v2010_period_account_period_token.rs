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
pub struct ApiPeriodV2010PeriodAccountPeriodToken {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Token resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The date and time in GMT that the resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT that the resource was last updated specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// An array representing the ephemeral credentials and the STUN and TURN server URIs.
    #[serde(rename = "ice_servers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ice_servers: Option<Option<Vec<crate::models::ApiV2010AccountTokenIceServersInner>>>,
    /// The temporary password that the username will use when authenticating with Twilio.
    #[serde(rename = "password", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub password: Option<Option<String>>,
    /// The duration in seconds for which the username and password are valid.
    #[serde(rename = "ttl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<Option<String>>,
    /// The temporary username that uniquely identifies a Token.
    #[serde(rename = "username", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub username: Option<Option<String>>,
}

impl Default for ApiPeriodV2010PeriodAccountPeriodToken {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiPeriodV2010PeriodAccountPeriodToken {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodToken {
        ApiPeriodV2010PeriodAccountPeriodToken {
            account_sid: None,
            date_created: None,
            date_updated: None,
            ice_servers: None,
            password: None,
            ttl: None,
            username: None,
        }
    }
}


