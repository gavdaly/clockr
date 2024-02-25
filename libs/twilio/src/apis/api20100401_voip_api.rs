/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.51.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`list_available_phone_number_voip`]
#[derive(Clone, Debug)]
pub struct ListAvailablePhoneNumberVoipParams {
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
    pub page_token: Option<String>
}


/// struct for typed errors of method [`list_available_phone_number_voip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAvailablePhoneNumberVoipError {
    UnknownValue(serde_json::Value),
}


/// 
pub async fn list_available_phone_number_voip(configuration: &configuration::Configuration, params: ListAvailablePhoneNumberVoipParams) -> Result<crate::models::ListAvailablePhoneNumberVoipResponse, Error<ListAvailablePhoneNumberVoipError>> {
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

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/Voip.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), CountryCode=crate::apis::urlencode(country_code));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = area_code {
        local_var_req_builder = local_var_req_builder.query(&[("AreaCode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = contains {
        local_var_req_builder = local_var_req_builder.query(&[("Contains", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sms_enabled {
        local_var_req_builder = local_var_req_builder.query(&[("SmsEnabled", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = mms_enabled {
        local_var_req_builder = local_var_req_builder.query(&[("MmsEnabled", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = voice_enabled {
        local_var_req_builder = local_var_req_builder.query(&[("VoiceEnabled", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude_all_address_required {
        local_var_req_builder = local_var_req_builder.query(&[("ExcludeAllAddressRequired", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude_local_address_required {
        local_var_req_builder = local_var_req_builder.query(&[("ExcludeLocalAddressRequired", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude_foreign_address_required {
        local_var_req_builder = local_var_req_builder.query(&[("ExcludeForeignAddressRequired", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = beta {
        local_var_req_builder = local_var_req_builder.query(&[("Beta", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = near_number {
        local_var_req_builder = local_var_req_builder.query(&[("NearNumber", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = near_lat_long {
        local_var_req_builder = local_var_req_builder.query(&[("NearLatLong", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = distance {
        local_var_req_builder = local_var_req_builder.query(&[("Distance", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = in_postal_code {
        local_var_req_builder = local_var_req_builder.query(&[("InPostalCode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = in_region {
        local_var_req_builder = local_var_req_builder.query(&[("InRegion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = in_rate_center {
        local_var_req_builder = local_var_req_builder.query(&[("InRateCenter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = in_lata {
        local_var_req_builder = local_var_req_builder.query(&[("InLata", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = in_locality {
        local_var_req_builder = local_var_req_builder.query(&[("InLocality", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fax_enabled {
        local_var_req_builder = local_var_req_builder.query(&[("FaxEnabled", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("Page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder = local_var_req_builder.query(&[("PageToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListAvailablePhoneNumberVoipError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

