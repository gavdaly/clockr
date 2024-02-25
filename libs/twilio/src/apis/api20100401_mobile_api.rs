/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.51.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`create_incoming_phone_number_mobile`]
#[derive(Clone, Debug)]
pub struct CreateIncomingPhoneNumberMobileParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource.
    pub account_sid: String,
    /// The phone number to purchase specified in [E.164](https://www.twilio.com/docs/glossary/what-e164) format.  E.164 phone numbers consist of a + followed by the country code and subscriber number without punctuation characters. For example, +14155551234.
    pub phone_number: String,
    /// The API version to use for incoming calls made to the new phone number. The default is `2010-04-01`.
    pub api_version: Option<String>,
    /// A descriptive string that you created to describe the new phone number. It can be up to 64 characters long. By default, the is a formatted version of the phone number.
    pub friendly_name: Option<String>,
    /// The SID of the application that should handle SMS messages sent to the new phone number. If an `sms_application_sid` is present, we ignore all of the `sms_*_url` urls and use those of the application.
    pub sms_application_sid: Option<String>,
    /// The HTTP method that we should use to call `sms_fallback_url`. Can be: `GET` or `POST` and defaults to `POST`.
    pub sms_fallback_method: Option<String>,
    /// The URL that we should call when an error occurs while requesting or executing the TwiML defined by `sms_url`.
    pub sms_fallback_url: Option<String>,
    /// The HTTP method that we should use to call `sms_url`. Can be: `GET` or `POST` and defaults to `POST`.
    pub sms_method: Option<String>,
    /// The URL we should call when the new phone number receives an incoming SMS message.
    pub sms_url: Option<String>,
    /// The URL we should call using the `status_callback_method` to send status information to your application.
    pub status_callback: Option<String>,
    /// The HTTP method we should use to call `status_callback`. Can be: `GET` or `POST` and defaults to `POST`.
    pub status_callback_method: Option<String>,
    /// The SID of the application we should use to handle calls to the new phone number. If a `voice_application_sid` is present, we ignore all of the voice urls and use only those set on the application. Setting a `voice_application_sid` will automatically delete your `trunk_sid` and vice versa.
    pub voice_application_sid: Option<String>,
    /// Whether to lookup the caller's name from the CNAM database and post it to your app. Can be: `true` or `false` and defaults to `false`.
    pub voice_caller_id_lookup: Option<bool>,
    /// The HTTP method that we should use to call `voice_fallback_url`. Can be: `GET` or `POST` and defaults to `POST`.
    pub voice_fallback_method: Option<String>,
    /// The URL that we should call when an error occurs retrieving or executing the TwiML requested by `url`.
    pub voice_fallback_url: Option<String>,
    /// The HTTP method that we should use to call `voice_url`. Can be: `GET` or `POST` and defaults to `POST`.
    pub voice_method: Option<String>,
    /// The URL that we should call to answer a call to the new phone number. The `voice_url` will not be called if a `voice_application_sid` or a `trunk_sid` is set.
    pub voice_url: Option<String>,
    /// The SID of the Identity resource that we should associate with the new phone number. Some regions require an identity to meet local regulations.
    pub identity_sid: Option<String>,
    /// The SID of the Address resource we should associate with the new phone number. Some regions require addresses to meet local regulations.
    pub address_sid: Option<String>,
    pub emergency_status: Option<String>,
    /// The SID of the emergency address configuration to use for emergency calling from the new phone number.
    pub emergency_address_sid: Option<String>,
    /// The SID of the Trunk we should use to handle calls to the new phone number. If a `trunk_sid` is present, we ignore all of the voice urls and voice applications and use only those set on the Trunk. Setting a `trunk_sid` will automatically delete your `voice_application_sid` and vice versa.
    pub trunk_sid: Option<String>,
    pub voice_receive_mode: Option<String>,
    /// The SID of the Bundle resource that you associate with the phone number. Some regions require a Bundle to meet local Regulations.
    pub bundle_sid: Option<String>,
}

/// struct for passing parameters to the method [`list_available_phone_number_mobile`]
#[derive(Clone, Debug)]
pub struct ListAvailablePhoneNumberMobileParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the AvailablePhoneNumber resources.
    pub account_sid: String,
    /// The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country from which to read phone numbers.
    pub country_code: String,
    /// The area code of the phone numbers to read. Applies to only phone numbers in the US and Canada.
    pub area_code: Option<i32>,
    /// The pattern on which to match phone numbers. Valid characters are `*`, `0-9`, `a-z`, and `A-Z`. The `*` character matches any single digit. For examples, see [Example 2](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-2) and [Example 3](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-3). If specified, this value must have at least two characters.
    pub contains: Option<String>,
    /// Whether the phone numbers can receive text messages. Can be: `true` or `false`.
    pub sms_enabled: Option<bool>,
    /// Whether the phone numbers can receive MMS messages. Can be: `true` or `false`.
    pub mms_enabled: Option<bool>,
    /// Whether the phone numbers can receive calls. Can be: `true` or `false`.
    pub voice_enabled: Option<bool>,
    /// Whether to exclude phone numbers that require an [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.
    pub exclude_all_address_required: Option<bool>,
    /// Whether to exclude phone numbers that require a local [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.
    pub exclude_local_address_required: Option<bool>,
    /// Whether to exclude phone numbers that require a foreign [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.
    pub exclude_foreign_address_required: Option<bool>,
    /// Whether to read phone numbers that are new to the Twilio platform. Can be: `true` or `false` and the default is `true`.
    pub beta: Option<bool>,
    /// Given a phone number, find a geographically close number within `distance` miles. Distance defaults to 25 miles. Applies to only phone numbers in the US and Canada.
    pub near_number: Option<String>,
    /// Given a latitude/longitude pair `lat,long` find geographically close numbers within `distance` miles. Applies to only phone numbers in the US and Canada.
    pub near_lat_long: Option<String>,
    /// The search radius, in miles, for a `near_` query.  Can be up to `500` and the default is `25`. Applies to only phone numbers in the US and Canada.
    pub distance: Option<i32>,
    /// Limit results to a particular postal code. Given a phone number, search within the same postal code as that number. Applies to only phone numbers in the US and Canada.
    pub in_postal_code: Option<String>,
    /// Limit results to a particular region, state, or province. Given a phone number, search within the same region as that number. Applies to only phone numbers in the US and Canada.
    pub in_region: Option<String>,
    /// Limit results to a specific rate center, or given a phone number search within the same rate center as that number. Requires `in_lata` to be set as well. Applies to only phone numbers in the US and Canada.
    pub in_rate_center: Option<String>,
    /// Limit results to a specific local access and transport area ([LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area)). Given a phone number, search within the same [LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area) as that number. Applies to only phone numbers in the US and Canada.
    pub in_lata: Option<String>,
    /// Limit results to a particular locality or city. Given a phone number, search within the same Locality as that number.
    pub in_locality: Option<String>,
    /// Whether the phone numbers can receive faxes. Can be: `true` or `false`.
    pub fax_enabled: Option<bool>,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>,
}

/// struct for passing parameters to the method [`list_incoming_phone_number_mobile`]
#[derive(Clone, Debug)]
pub struct ListIncomingPhoneNumberMobileParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to read.
    pub account_sid: String,
    /// Whether to include phone numbers new to the Twilio platform. Can be: `true` or `false` and the default is `true`.
    pub beta: Option<bool>,
    /// A string that identifies the resources to read.
    pub friendly_name: Option<String>,
    /// The phone numbers of the IncomingPhoneNumber resources to read. You can specify partial numbers and use '*' as a wildcard for any digit.
    pub phone_number: Option<String>,
    /// Whether to include phone numbers based on their origin. Can be: `twilio` or `hosted`. By default, phone numbers of all origin are included.
    pub origin: Option<String>,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>,
}

/// struct for typed errors of method [`create_incoming_phone_number_mobile`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIncomingPhoneNumberMobileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_available_phone_number_mobile`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAvailablePhoneNumberMobileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_incoming_phone_number_mobile`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncomingPhoneNumberMobileError {
    UnknownValue(serde_json::Value),
}

///
pub async fn create_incoming_phone_number_mobile(configuration: &configuration::Configuration, params: CreateIncomingPhoneNumberMobileParams) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberMobile, Error<CreateIncomingPhoneNumberMobileError>>{
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let phone_number = params.phone_number;
    let api_version = params.api_version;
    let friendly_name = params.friendly_name;
    let sms_application_sid = params.sms_application_sid;
    let sms_fallback_method = params.sms_fallback_method;
    let sms_fallback_url = params.sms_fallback_url;
    let sms_method = params.sms_method;
    let sms_url = params.sms_url;
    let status_callback = params.status_callback;
    let status_callback_method = params.status_callback_method;
    let voice_application_sid = params.voice_application_sid;
    let voice_caller_id_lookup = params.voice_caller_id_lookup;
    let voice_fallback_method = params.voice_fallback_method;
    let voice_fallback_url = params.voice_fallback_url;
    let voice_method = params.voice_method;
    let voice_url = params.voice_url;
    let identity_sid = params.identity_sid;
    let address_sid = params.address_sid;
    let emergency_status = params.emergency_status;
    let emergency_address_sid = params.emergency_address_sid;
    let trunk_sid = params.trunk_sid;
    let voice_receive_mode = params.voice_receive_mode;
    let bundle_sid = params.bundle_sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Mobile.json",
        local_var_configuration.base_path,
        AccountSid = crate::apis::urlencode(account_sid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("PhoneNumber", phone_number.to_string());
    if let Some(local_var_param_value) = api_version {
        local_var_form_params.insert("ApiVersion", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = friendly_name {
        local_var_form_params.insert("FriendlyName", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_application_sid {
        local_var_form_params.insert("SmsApplicationSid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_fallback_method {
        local_var_form_params.insert("SmsFallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_fallback_url {
        local_var_form_params.insert("SmsFallbackUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_method {
        local_var_form_params.insert("SmsMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_url {
        local_var_form_params.insert("SmsUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback {
        local_var_form_params.insert("StatusCallback", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback_method {
        local_var_form_params.insert("StatusCallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_application_sid {
        local_var_form_params.insert("VoiceApplicationSid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_caller_id_lookup {
        local_var_form_params.insert("VoiceCallerIdLookup", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_fallback_method {
        local_var_form_params.insert("VoiceFallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_fallback_url {
        local_var_form_params.insert("VoiceFallbackUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_method {
        local_var_form_params.insert("VoiceMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_url {
        local_var_form_params.insert("VoiceUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = identity_sid {
        local_var_form_params.insert("IdentitySid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = address_sid {
        local_var_form_params.insert("AddressSid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = emergency_status {
        local_var_form_params.insert("EmergencyStatus", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = emergency_address_sid {
        local_var_form_params.insert("EmergencyAddressSid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = trunk_sid {
        local_var_form_params.insert("TrunkSid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_receive_mode {
        local_var_form_params.insert("VoiceReceiveMode", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = bundle_sid {
        local_var_form_params.insert("BundleSid", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateIncomingPhoneNumberMobileError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

///
pub async fn list_available_phone_number_mobile(
    configuration: &configuration::Configuration,
    params: ListAvailablePhoneNumberMobileParams,
) -> Result<
    crate::models::ListAvailablePhoneNumberMobileResponse,
    Error<ListAvailablePhoneNumberMobileError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let country_code = params.country_code;
    let area_code = params.area_code;
    let contains = params.contains;
    let sms_enabled = params.sms_enabled;
    let mms_enabled = params.mms_enabled;
    let voice_enabled = params.voice_enabled;
    let exclude_all_address_required = params.exclude_all_address_required;
    let exclude_local_address_required = params.exclude_local_address_required;
    let exclude_foreign_address_required = params.exclude_foreign_address_required;
    let beta = params.beta;
    let near_number = params.near_number;
    let near_lat_long = params.near_lat_long;
    let distance = params.distance;
    let in_postal_code = params.in_postal_code;
    let in_region = params.in_region;
    let in_rate_center = params.in_rate_center;
    let in_lata = params.in_lata;
    let in_locality = params.in_locality;
    let fax_enabled = params.fax_enabled;
    let page_size = params.page_size;
    let page = params.page;
    let page_token = params.page_token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/Mobile.json",
        local_var_configuration.base_path,
        AccountSid = crate::apis::urlencode(account_sid),
        CountryCode = crate::apis::urlencode(country_code)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = area_code {
        local_var_req_builder =
            local_var_req_builder.query(&[("AreaCode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = contains {
        local_var_req_builder =
            local_var_req_builder.query(&[("Contains", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sms_enabled {
        local_var_req_builder =
            local_var_req_builder.query(&[("SmsEnabled", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = mms_enabled {
        local_var_req_builder =
            local_var_req_builder.query(&[("MmsEnabled", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = voice_enabled {
        local_var_req_builder =
            local_var_req_builder.query(&[("VoiceEnabled", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude_all_address_required {
        local_var_req_builder = local_var_req_builder
            .query(&[("ExcludeAllAddressRequired", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude_local_address_required {
        local_var_req_builder = local_var_req_builder
            .query(&[("ExcludeLocalAddressRequired", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude_foreign_address_required {
        local_var_req_builder = local_var_req_builder
            .query(&[("ExcludeForeignAddressRequired", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = beta {
        local_var_req_builder =
            local_var_req_builder.query(&[("Beta", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = near_number {
        local_var_req_builder =
            local_var_req_builder.query(&[("NearNumber", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = near_lat_long {
        local_var_req_builder =
            local_var_req_builder.query(&[("NearLatLong", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = distance {
        local_var_req_builder =
            local_var_req_builder.query(&[("Distance", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = in_postal_code {
        local_var_req_builder =
            local_var_req_builder.query(&[("InPostalCode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = in_region {
        local_var_req_builder =
            local_var_req_builder.query(&[("InRegion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = in_rate_center {
        local_var_req_builder =
            local_var_req_builder.query(&[("InRateCenter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = in_lata {
        local_var_req_builder =
            local_var_req_builder.query(&[("InLata", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = in_locality {
        local_var_req_builder =
            local_var_req_builder.query(&[("InLocality", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fax_enabled {
        local_var_req_builder =
            local_var_req_builder.query(&[("FaxEnabled", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("Page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("PageToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListAvailablePhoneNumberMobileError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

///
pub async fn list_incoming_phone_number_mobile(
    configuration: &configuration::Configuration,
    params: ListIncomingPhoneNumberMobileParams,
) -> Result<
    crate::models::ListIncomingPhoneNumberMobileResponse,
    Error<ListIncomingPhoneNumberMobileError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let beta = params.beta;
    let friendly_name = params.friendly_name;
    let phone_number = params.phone_number;
    let origin = params.origin;
    let page_size = params.page_size;
    let page = params.page;
    let page_token = params.page_token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Mobile.json",
        local_var_configuration.base_path,
        AccountSid = crate::apis::urlencode(account_sid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = beta {
        local_var_req_builder =
            local_var_req_builder.query(&[("Beta", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = friendly_name {
        local_var_req_builder =
            local_var_req_builder.query(&[("FriendlyName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = phone_number {
        local_var_req_builder =
            local_var_req_builder.query(&[("PhoneNumber", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = origin {
        local_var_req_builder =
            local_var_req_builder.query(&[("Origin", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("Page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("PageToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListIncomingPhoneNumberMobileError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
