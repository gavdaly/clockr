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

/// struct for passing parameters to the method [`create_sip_domain`]
#[derive(Clone, Debug)]
pub struct CreateSipDomainParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource.
    pub account_sid: String,
    /// The unique address you reserve on Twilio to which you route your SIP traffic. Domain names can contain letters, digits, and \\\"-\\\" and must end with `sip.twilio.com`.
    pub domain_name: String,
    /// A descriptive string that you created to describe the resource. It can be up to 64 characters long.
    pub friendly_name: Option<String>,
    /// The URL we should when the domain receives a call.
    pub voice_url: Option<String>,
    /// The HTTP method we should use to call `voice_url`. Can be: `GET` or `POST`.
    pub voice_method: Option<String>,
    /// The URL that we should call when an error occurs while retrieving or executing the TwiML from `voice_url`.
    pub voice_fallback_url: Option<String>,
    /// The HTTP method we should use to call `voice_fallback_url`. Can be: `GET` or `POST`.
    pub voice_fallback_method: Option<String>,
    /// The URL that we should call to pass status parameters (such as call ended) to your application.
    pub voice_status_callback_url: Option<String>,
    /// The HTTP method we should use to call `voice_status_callback_url`. Can be: `GET` or `POST`.
    pub voice_status_callback_method: Option<String>,
    /// Whether to allow SIP Endpoints to register with the domain to receive calls. Can be `true` or `false`. `true` allows SIP Endpoints to register with the domain to receive calls, `false` does not.
    pub sip_registration: Option<bool>,
    /// Whether emergency calling is enabled for the domain. If enabled, allows emergency calls on the domain from phone numbers with validated addresses.
    pub emergency_calling_enabled: Option<bool>,
    /// Whether secure SIP is enabled for the domain. If enabled, TLS will be enforced and SRTP will be negotiated on all incoming calls to this sip domain.
    pub secure: Option<bool>,
    /// The SID of the BYOC Trunk(Bring Your Own Carrier) resource that the Sip Domain will be associated with.
    pub byoc_trunk_sid: Option<String>,
    /// Whether an emergency caller sid is configured for the domain. If present, this phone number will be used as the callback for the emergency call.
    pub emergency_caller_sid: Option<String>,
}

/// struct for passing parameters to the method [`delete_sip_domain`]
#[derive(Clone, Debug)]
pub struct DeleteSipDomainParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the SipDomain resources to delete.
    pub account_sid: String,
    /// The Twilio-provided string that uniquely identifies the SipDomain resource to delete.
    pub sid: String,
}

/// struct for passing parameters to the method [`fetch_sip_domain`]
#[derive(Clone, Debug)]
pub struct FetchSipDomainParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the SipDomain resource to fetch.
    pub account_sid: String,
    /// The Twilio-provided string that uniquely identifies the SipDomain resource to fetch.
    pub sid: String,
}

/// struct for passing parameters to the method [`list_sip_domain`]
#[derive(Clone, Debug)]
pub struct ListSipDomainParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the SipDomain resources to read.
    pub account_sid: String,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>,
}

/// struct for passing parameters to the method [`update_sip_domain`]
#[derive(Clone, Debug)]
pub struct UpdateSipDomainParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the SipDomain resource to update.
    pub account_sid: String,
    /// The Twilio-provided string that uniquely identifies the SipDomain resource to update.
    pub sid: String,
    /// A descriptive string that you created to describe the resource. It can be up to 64 characters long.
    pub friendly_name: Option<String>,
    /// The HTTP method we should use to call `voice_fallback_url`. Can be: `GET` or `POST`.
    pub voice_fallback_method: Option<String>,
    /// The URL that we should call when an error occurs while retrieving or executing the TwiML requested by `voice_url`.
    pub voice_fallback_url: Option<String>,
    /// The HTTP method we should use to call `voice_url`
    pub voice_method: Option<String>,
    /// The HTTP method we should use to call `voice_status_callback_url`. Can be: `GET` or `POST`.
    pub voice_status_callback_method: Option<String>,
    /// The URL that we should call to pass status parameters (such as call ended) to your application.
    pub voice_status_callback_url: Option<String>,
    /// The URL we should call when the domain receives a call.
    pub voice_url: Option<String>,
    /// Whether to allow SIP Endpoints to register with the domain to receive calls. Can be `true` or `false`. `true` allows SIP Endpoints to register with the domain to receive calls, `false` does not.
    pub sip_registration: Option<bool>,
    /// The unique address you reserve on Twilio to which you route your SIP traffic. Domain names can contain letters, digits, and \\\"-\\\" and must end with `sip.twilio.com`.
    pub domain_name: Option<String>,
    /// Whether emergency calling is enabled for the domain. If enabled, allows emergency calls on the domain from phone numbers with validated addresses.
    pub emergency_calling_enabled: Option<bool>,
    /// Whether secure SIP is enabled for the domain. If enabled, TLS will be enforced and SRTP will be negotiated on all incoming calls to this sip domain.
    pub secure: Option<bool>,
    /// The SID of the BYOC Trunk(Bring Your Own Carrier) resource that the Sip Domain will be associated with.
    pub byoc_trunk_sid: Option<String>,
    /// Whether an emergency caller sid is configured for the domain. If present, this phone number will be used as the callback for the emergency call.
    pub emergency_caller_sid: Option<String>,
}

/// struct for typed errors of method [`create_sip_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSipDomainError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_sip_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSipDomainError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_sip_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchSipDomainError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_sip_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSipDomainError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_sip_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSipDomainError {
    UnknownValue(serde_json::Value),
}

/// Create a new Domain
pub async fn create_sip_domain(
    configuration: &configuration::Configuration,
    params: CreateSipDomainParams,
) -> Result<
    crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomain,
    Error<CreateSipDomainError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let domain_name = params.domain_name;
    let friendly_name = params.friendly_name;
    let voice_url = params.voice_url;
    let voice_method = params.voice_method;
    let voice_fallback_url = params.voice_fallback_url;
    let voice_fallback_method = params.voice_fallback_method;
    let voice_status_callback_url = params.voice_status_callback_url;
    let voice_status_callback_method = params.voice_status_callback_method;
    let sip_registration = params.sip_registration;
    let emergency_calling_enabled = params.emergency_calling_enabled;
    let secure = params.secure;
    let byoc_trunk_sid = params.byoc_trunk_sid;
    let emergency_caller_sid = params.emergency_caller_sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/SIP/Domains.json",
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
    local_var_form_params.insert("DomainName", domain_name.to_string());
    if let Some(local_var_param_value) = friendly_name {
        local_var_form_params.insert("FriendlyName", local_var_param_value.to_string());
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
    if let Some(local_var_param_value) = voice_status_callback_url {
        local_var_form_params.insert("VoiceStatusCallbackUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_status_callback_method {
        local_var_form_params.insert(
            "VoiceStatusCallbackMethod",
            local_var_param_value.to_string(),
        );
    }
    if let Some(local_var_param_value) = sip_registration {
        local_var_form_params.insert("SipRegistration", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = emergency_calling_enabled {
        local_var_form_params.insert("EmergencyCallingEnabled", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = secure {
        local_var_form_params.insert("Secure", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = byoc_trunk_sid {
        local_var_form_params.insert("ByocTrunkSid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = emergency_caller_sid {
        local_var_form_params.insert("EmergencyCallerSid", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateSipDomainError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete an instance of a Domain
pub async fn delete_sip_domain(
    configuration: &configuration::Configuration,
    params: DeleteSipDomainParams,
) -> Result<(), Error<DeleteSipDomainError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let sid = params.sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{Sid}.json",
        local_var_configuration.base_path,
        AccountSid = crate::apis::urlencode(account_sid),
        Sid = crate::apis::urlencode(sid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        Ok(())
    } else {
        let local_var_entity: Option<DeleteSipDomainError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch an instance of a Domain
pub async fn fetch_sip_domain(
    configuration: &configuration::Configuration,
    params: FetchSipDomainParams,
) -> Result<
    crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomain,
    Error<FetchSipDomainError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let sid = params.sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{Sid}.json",
        local_var_configuration.base_path,
        AccountSid = crate::apis::urlencode(account_sid),
        Sid = crate::apis::urlencode(sid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<FetchSipDomainError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a list of domains belonging to the account used to make the request
pub async fn list_sip_domain(
    configuration: &configuration::Configuration,
    params: ListSipDomainParams,
) -> Result<crate::models::ListSipDomainResponse, Error<ListSipDomainError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let page_size = params.page_size;
    let page = params.page;
    let page_token = params.page_token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/SIP/Domains.json",
        local_var_configuration.base_path,
        AccountSid = crate::apis::urlencode(account_sid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<ListSipDomainError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the attributes of a domain
pub async fn update_sip_domain(
    configuration: &configuration::Configuration,
    params: UpdateSipDomainParams,
) -> Result<
    crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomain,
    Error<UpdateSipDomainError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let sid = params.sid;
    let friendly_name = params.friendly_name;
    let voice_fallback_method = params.voice_fallback_method;
    let voice_fallback_url = params.voice_fallback_url;
    let voice_method = params.voice_method;
    let voice_status_callback_method = params.voice_status_callback_method;
    let voice_status_callback_url = params.voice_status_callback_url;
    let voice_url = params.voice_url;
    let sip_registration = params.sip_registration;
    let domain_name = params.domain_name;
    let emergency_calling_enabled = params.emergency_calling_enabled;
    let secure = params.secure;
    let byoc_trunk_sid = params.byoc_trunk_sid;
    let emergency_caller_sid = params.emergency_caller_sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{Sid}.json",
        local_var_configuration.base_path,
        AccountSid = crate::apis::urlencode(account_sid),
        Sid = crate::apis::urlencode(sid)
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
    if let Some(local_var_param_value) = friendly_name {
        local_var_form_params.insert("FriendlyName", local_var_param_value.to_string());
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
    if let Some(local_var_param_value) = voice_status_callback_method {
        local_var_form_params.insert(
            "VoiceStatusCallbackMethod",
            local_var_param_value.to_string(),
        );
    }
    if let Some(local_var_param_value) = voice_status_callback_url {
        local_var_form_params.insert("VoiceStatusCallbackUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_url {
        local_var_form_params.insert("VoiceUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sip_registration {
        local_var_form_params.insert("SipRegistration", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = domain_name {
        local_var_form_params.insert("DomainName", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = emergency_calling_enabled {
        local_var_form_params.insert("EmergencyCallingEnabled", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = secure {
        local_var_form_params.insert("Secure", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = byoc_trunk_sid {
        local_var_form_params.insert("ByocTrunkSid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = emergency_caller_sid {
        local_var_form_params.insert("EmergencyCallerSid", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateSipDomainError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
