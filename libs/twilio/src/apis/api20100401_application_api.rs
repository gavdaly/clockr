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

/// struct for passing parameters to the method [`create_application`]
#[derive(Clone, Debug)]
pub struct CreateApplicationParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource.
    pub account_sid: String,
    /// The API version to use to start a new TwiML session. Can be: `2010-04-01` or `2008-08-01`. The default value is the account's default API version.
    pub api_version: Option<String>,
    /// The URL we should call when the phone number assigned to this application receives a call.
    pub voice_url: Option<String>,
    /// The HTTP method we should use to call `voice_url`. Can be: `GET` or `POST`.
    pub voice_method: Option<String>,
    /// The URL that we should call when an error occurs retrieving or executing the TwiML requested by `url`.
    pub voice_fallback_url: Option<String>,
    /// The HTTP method we should use to call `voice_fallback_url`. Can be: `GET` or `POST`.
    pub voice_fallback_method: Option<String>,
    /// The URL we should call using the `status_callback_method` to send status information to your application.
    pub status_callback: Option<String>,
    /// The HTTP method we should use to call `status_callback`. Can be: `GET` or `POST`.
    pub status_callback_method: Option<String>,
    /// Whether we should look up the caller's caller-ID name from the CNAM database (additional charges apply). Can be: `true` or `false`.
    pub voice_caller_id_lookup: Option<bool>,
    /// The URL we should call when the phone number receives an incoming SMS message.
    pub sms_url: Option<String>,
    /// The HTTP method we should use to call `sms_url`. Can be: `GET` or `POST`.
    pub sms_method: Option<String>,
    /// The URL that we should call when an error occurs while retrieving or executing the TwiML from `sms_url`.
    pub sms_fallback_url: Option<String>,
    /// The HTTP method we should use to call `sms_fallback_url`. Can be: `GET` or `POST`.
    pub sms_fallback_method: Option<String>,
    /// The URL we should call using a POST method to send status information about SMS messages sent by the application.
    pub sms_status_callback: Option<String>,
    /// The URL we should call using a POST method to send message status information to your application.
    pub message_status_callback: Option<String>,
    /// A descriptive string that you create to describe the new application. It can be up to 64 characters long.
    pub friendly_name: Option<String>,
    /// Whether to allow other Twilio accounts to dial this applicaton using Dial verb. Can be: `true` or `false`.
    pub public_application_connect_enabled: Option<bool>
}

/// struct for passing parameters to the method [`delete_application`]
#[derive(Clone, Debug)]
pub struct DeleteApplicationParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Application resources to delete.
    pub account_sid: String,
    /// The Twilio-provided string that uniquely identifies the Application resource to delete.
    pub sid: String
}

/// struct for passing parameters to the method [`fetch_application`]
#[derive(Clone, Debug)]
pub struct FetchApplicationParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Application resource to fetch.
    pub account_sid: String,
    /// The Twilio-provided string that uniquely identifies the Application resource to fetch.
    pub sid: String
}

/// struct for passing parameters to the method [`list_application`]
#[derive(Clone, Debug)]
pub struct ListApplicationParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Application resources to read.
    pub account_sid: String,
    /// The string that identifies the Application resources to read.
    pub friendly_name: Option<String>,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>
}

/// struct for passing parameters to the method [`update_application`]
#[derive(Clone, Debug)]
pub struct UpdateApplicationParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Application resources to update.
    pub account_sid: String,
    /// The Twilio-provided string that uniquely identifies the Application resource to update.
    pub sid: String,
    /// A descriptive string that you create to describe the resource. It can be up to 64 characters long.
    pub friendly_name: Option<String>,
    /// The API version to use to start a new TwiML session. Can be: `2010-04-01` or `2008-08-01`. The default value is your account's default API version.
    pub api_version: Option<String>,
    /// The URL we should call when the phone number assigned to this application receives a call.
    pub voice_url: Option<String>,
    /// The HTTP method we should use to call `voice_url`. Can be: `GET` or `POST`.
    pub voice_method: Option<String>,
    /// The URL that we should call when an error occurs retrieving or executing the TwiML requested by `url`.
    pub voice_fallback_url: Option<String>,
    /// The HTTP method we should use to call `voice_fallback_url`. Can be: `GET` or `POST`.
    pub voice_fallback_method: Option<String>,
    /// The URL we should call using the `status_callback_method` to send status information to your application.
    pub status_callback: Option<String>,
    /// The HTTP method we should use to call `status_callback`. Can be: `GET` or `POST`.
    pub status_callback_method: Option<String>,
    /// Whether we should look up the caller's caller-ID name from the CNAM database (additional charges apply). Can be: `true` or `false`.
    pub voice_caller_id_lookup: Option<bool>,
    /// The URL we should call when the phone number receives an incoming SMS message.
    pub sms_url: Option<String>,
    /// The HTTP method we should use to call `sms_url`. Can be: `GET` or `POST`.
    pub sms_method: Option<String>,
    /// The URL that we should call when an error occurs while retrieving or executing the TwiML from `sms_url`.
    pub sms_fallback_url: Option<String>,
    /// The HTTP method we should use to call `sms_fallback_url`. Can be: `GET` or `POST`.
    pub sms_fallback_method: Option<String>,
    /// Same as message_status_callback: The URL we should call using a POST method to send status information about SMS messages sent by the application. Deprecated, included for backwards compatibility.
    pub sms_status_callback: Option<String>,
    /// The URL we should call using a POST method to send message status information to your application.
    pub message_status_callback: Option<String>,
    /// Whether to allow other Twilio accounts to dial this applicaton using Dial verb. Can be: `true` or `false`.
    pub public_application_connect_enabled: Option<bool>
}


/// struct for typed errors of method [`create_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApplicationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchApplicationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApplicationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationError {
    UnknownValue(serde_json::Value),
}


/// Create a new application within your account
pub async fn create_application(configuration: &configuration::Configuration, params: CreateApplicationParams) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodApplication, Error<CreateApplicationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let api_version = params.api_version;
    let voice_url = params.voice_url;
    let voice_method = params.voice_method;
    let voice_fallback_url = params.voice_fallback_url;
    let voice_fallback_method = params.voice_fallback_method;
    let status_callback = params.status_callback;
    let status_callback_method = params.status_callback_method;
    let voice_caller_id_lookup = params.voice_caller_id_lookup;
    let sms_url = params.sms_url;
    let sms_method = params.sms_method;
    let sms_fallback_url = params.sms_fallback_url;
    let sms_fallback_method = params.sms_fallback_method;
    let sms_status_callback = params.sms_status_callback;
    let message_status_callback = params.message_status_callback;
    let friendly_name = params.friendly_name;
    let public_application_connect_enabled = params.public_application_connect_enabled;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Applications.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = api_version {
        local_var_form_params.insert("ApiVersion", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_url {
        local_var_form_params.insert("VoiceUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_method {
        local_var_form_params.insert("VoiceMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_fallback_url {
        local_var_form_params.insert("VoiceFallbackUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_fallback_method {
        local_var_form_params.insert("VoiceFallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback {
        local_var_form_params.insert("StatusCallback", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback_method {
        local_var_form_params.insert("StatusCallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_caller_id_lookup {
        local_var_form_params.insert("VoiceCallerIdLookup", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_url {
        local_var_form_params.insert("SmsUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_method {
        local_var_form_params.insert("SmsMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_fallback_url {
        local_var_form_params.insert("SmsFallbackUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_fallback_method {
        local_var_form_params.insert("SmsFallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_status_callback {
        local_var_form_params.insert("SmsStatusCallback", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = message_status_callback {
        local_var_form_params.insert("MessageStatusCallback", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = friendly_name {
        local_var_form_params.insert("FriendlyName", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = public_application_connect_enabled {
        local_var_form_params.insert("PublicApplicationConnectEnabled", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateApplicationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete the application by the specified application sid
pub async fn delete_application(configuration: &configuration::Configuration, params: DeleteApplicationParams) -> Result<(), Error<DeleteApplicationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let sid = params.sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        Ok(())
    } else {
        let local_var_entity: Option<DeleteApplicationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch the application specified by the provided sid
pub async fn fetch_application(configuration: &configuration::Configuration, params: FetchApplicationParams) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodApplication, Error<FetchApplicationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let sid = params.sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<FetchApplicationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a list of applications representing an application within the requesting account
pub async fn list_application(configuration: &configuration::Configuration, params: ListApplicationParams) -> Result<crate::models::ListApplicationResponse, Error<ListApplicationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let friendly_name = params.friendly_name;
    let page_size = params.page_size;
    let page = params.page;
    let page_token = params.page_token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Applications.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = friendly_name {
        local_var_req_builder = local_var_req_builder.query(&[("FriendlyName", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListApplicationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the application's properties
pub async fn update_application(configuration: &configuration::Configuration, params: UpdateApplicationParams) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodApplication, Error<UpdateApplicationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let sid = params.sid;
    let friendly_name = params.friendly_name;
    let api_version = params.api_version;
    let voice_url = params.voice_url;
    let voice_method = params.voice_method;
    let voice_fallback_url = params.voice_fallback_url;
    let voice_fallback_method = params.voice_fallback_method;
    let status_callback = params.status_callback;
    let status_callback_method = params.status_callback_method;
    let voice_caller_id_lookup = params.voice_caller_id_lookup;
    let sms_url = params.sms_url;
    let sms_method = params.sms_method;
    let sms_fallback_url = params.sms_fallback_url;
    let sms_fallback_method = params.sms_fallback_method;
    let sms_status_callback = params.sms_status_callback;
    let message_status_callback = params.message_status_callback;
    let public_application_connect_enabled = params.public_application_connect_enabled;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = friendly_name {
        local_var_form_params.insert("FriendlyName", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = api_version {
        local_var_form_params.insert("ApiVersion", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_url {
        local_var_form_params.insert("VoiceUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_method {
        local_var_form_params.insert("VoiceMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_fallback_url {
        local_var_form_params.insert("VoiceFallbackUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_fallback_method {
        local_var_form_params.insert("VoiceFallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback {
        local_var_form_params.insert("StatusCallback", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback_method {
        local_var_form_params.insert("StatusCallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_caller_id_lookup {
        local_var_form_params.insert("VoiceCallerIdLookup", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_url {
        local_var_form_params.insert("SmsUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_method {
        local_var_form_params.insert("SmsMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_fallback_url {
        local_var_form_params.insert("SmsFallbackUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_fallback_method {
        local_var_form_params.insert("SmsFallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_status_callback {
        local_var_form_params.insert("SmsStatusCallback", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = message_status_callback {
        local_var_form_params.insert("MessageStatusCallback", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = public_application_connect_enabled {
        local_var_form_params.insert("PublicApplicationConnectEnabled", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateApplicationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

