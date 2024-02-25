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
pub struct ApiPeriodV2010PeriodAccountPeriodApplication {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Application resource.
    #[serde(
        rename = "account_sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_sid: Option<Option<String>>,
    /// The API version used to start a new TwiML session.
    #[serde(
        rename = "api_version",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<Option<String>>,
    /// The date and time in GMT that the resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(
        rename = "date_created",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT that the resource was last updated specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(
        rename = "date_updated",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_updated: Option<Option<String>>,
    /// The string that you assigned to describe the resource.
    #[serde(
        rename = "friendly_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<Option<String>>,
    /// The URL we call using a POST method to send message status information to your application.
    #[serde(
        rename = "message_status_callback",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub message_status_callback: Option<Option<String>>,
    /// The unique string that that we created to identify the Application resource.
    #[serde(
        rename = "sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sid: Option<Option<String>>,
    /// The HTTP method we use to call `sms_fallback_url`. Can be: `GET` or `POST`.
    #[serde(
        rename = "sms_fallback_method",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sms_fallback_method: Option<Option<SmsFallbackMethod>>,
    /// The URL that we call when an error occurs while retrieving or executing the TwiML from `sms_url`.
    #[serde(
        rename = "sms_fallback_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sms_fallback_url: Option<Option<String>>,
    /// The HTTP method we use to call `sms_url`. Can be: `GET` or `POST`.
    #[serde(
        rename = "sms_method",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sms_method: Option<Option<SmsMethod>>,
    /// The URL we call using a POST method to send status information to your application about SMS messages that refer to the application.
    #[serde(
        rename = "sms_status_callback",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sms_status_callback: Option<Option<String>>,
    /// The URL we call when the phone number receives an incoming SMS message.
    #[serde(
        rename = "sms_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sms_url: Option<Option<String>>,
    /// The URL we call using the `status_callback_method` to send status information to your application.
    #[serde(
        rename = "status_callback",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_callback: Option<Option<String>>,
    /// The HTTP method we use to call `status_callback`. Can be: `GET` or `POST`.
    #[serde(
        rename = "status_callback_method",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_callback_method: Option<Option<StatusCallbackMethod>>,
    /// The URI of the resource, relative to `https://api.twilio.com`.
    #[serde(
        rename = "uri",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub uri: Option<Option<String>>,
    /// Whether we look up the caller's caller-ID name from the CNAM database (additional charges apply). Can be: `true` or `false`.
    #[serde(
        rename = "voice_caller_id_lookup",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub voice_caller_id_lookup: Option<Option<bool>>,
    /// The HTTP method we use to call `voice_fallback_url`. Can be: `GET` or `POST`.
    #[serde(
        rename = "voice_fallback_method",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub voice_fallback_method: Option<Option<VoiceFallbackMethod>>,
    /// The URL that we call when an error occurs retrieving or executing the TwiML requested by `url`.
    #[serde(
        rename = "voice_fallback_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub voice_fallback_url: Option<Option<String>>,
    /// The HTTP method we use to call `voice_url`. Can be: `GET` or `POST`.
    #[serde(
        rename = "voice_method",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub voice_method: Option<Option<VoiceMethod>>,
    /// The URL we call when the phone number assigned to this application receives a call.
    #[serde(
        rename = "voice_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub voice_url: Option<Option<String>>,
    /// Whether to allow other Twilio accounts to dial this applicaton using Dial verb. Can be: `true` or `false`.
    #[serde(
        rename = "public_application_connect_enabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_application_connect_enabled: Option<Option<bool>>,
}

impl Default for ApiPeriodV2010PeriodAccountPeriodApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiPeriodV2010PeriodAccountPeriodApplication {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodApplication {
        ApiPeriodV2010PeriodAccountPeriodApplication {
            account_sid: None,
            api_version: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            message_status_callback: None,
            sid: None,
            sms_fallback_method: None,
            sms_fallback_url: None,
            sms_method: None,
            sms_status_callback: None,
            sms_url: None,
            status_callback: None,
            status_callback_method: None,
            uri: None,
            voice_caller_id_lookup: None,
            voice_fallback_method: None,
            voice_fallback_url: None,
            voice_method: None,
            voice_url: None,
            public_application_connect_enabled: None,
        }
    }
}

/// The HTTP method we use to call `sms_fallback_url`. Can be: `GET` or `POST`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SmsFallbackMethod {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for SmsFallbackMethod {
    fn default() -> SmsFallbackMethod {
        Self::Head
    }
}
/// The HTTP method we use to call `sms_url`. Can be: `GET` or `POST`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SmsMethod {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for SmsMethod {
    fn default() -> SmsMethod {
        Self::Head
    }
}
/// The HTTP method we use to call `status_callback`. Can be: `GET` or `POST`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusCallbackMethod {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for StatusCallbackMethod {
    fn default() -> StatusCallbackMethod {
        Self::Head
    }
}
/// The HTTP method we use to call `voice_fallback_url`. Can be: `GET` or `POST`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoiceFallbackMethod {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for VoiceFallbackMethod {
    fn default() -> VoiceFallbackMethod {
        Self::Head
    }
}
/// The HTTP method we use to call `voice_url`. Can be: `GET` or `POST`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoiceMethod {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for VoiceMethod {
    fn default() -> VoiceMethod {
        Self::Head
    }
}
