/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.51.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_sip_auth_registrations_credential_list_mapping`]
#[derive(Clone, Debug)]
pub struct CreateSipAuthRegistrationsCredentialListMappingParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource.
    pub account_sid: String,
    /// The SID of the SIP domain that will contain the new resource.
    pub domain_sid: String,
    /// The SID of the CredentialList resource to map to the SIP domain.
    pub credential_list_sid: String
}

/// struct for passing parameters to the method [`delete_sip_auth_registrations_credential_list_mapping`]
#[derive(Clone, Debug)]
pub struct DeleteSipAuthRegistrationsCredentialListMappingParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the CredentialListMapping resources to delete.
    pub account_sid: String,
    /// The SID of the SIP domain that contains the resources to delete.
    pub domain_sid: String,
    /// The Twilio-provided string that uniquely identifies the CredentialListMapping resource to delete.
    pub sid: String
}

/// struct for passing parameters to the method [`fetch_sip_auth_registrations_credential_list_mapping`]
#[derive(Clone, Debug)]
pub struct FetchSipAuthRegistrationsCredentialListMappingParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the CredentialListMapping resource to fetch.
    pub account_sid: String,
    /// The SID of the SIP domain that contains the resource to fetch.
    pub domain_sid: String,
    /// The Twilio-provided string that uniquely identifies the CredentialListMapping resource to fetch.
    pub sid: String
}

/// struct for passing parameters to the method [`list_sip_auth_registrations_credential_list_mapping`]
#[derive(Clone, Debug)]
pub struct ListSipAuthRegistrationsCredentialListMappingParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the CredentialListMapping resources to read.
    pub account_sid: String,
    /// The SID of the SIP domain that contains the resources to read.
    pub domain_sid: String,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>
}


/// struct for typed errors of method [`create_sip_auth_registrations_credential_list_mapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSipAuthRegistrationsCredentialListMappingError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_sip_auth_registrations_credential_list_mapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSipAuthRegistrationsCredentialListMappingError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_sip_auth_registrations_credential_list_mapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchSipAuthRegistrationsCredentialListMappingError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_sip_auth_registrations_credential_list_mapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSipAuthRegistrationsCredentialListMappingError {
    UnknownValue(serde_json::Value),
}


/// Create a new credential list mapping resource
pub async fn create_sip_auth_registrations_credential_list_mapping(configuration: &configuration::Configuration, params: CreateSipAuthRegistrationsCredentialListMappingParams) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthRegistrationsPeriodSipAuthRegistrationsCredentialListMapping, Error<CreateSipAuthRegistrationsCredentialListMappingError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let domain_sid = params.domain_sid;
    let credential_list_sid = params.credential_list_sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), DomainSid=crate::apis::urlencode(domain_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("CredentialListSid", credential_list_sid.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateSipAuthRegistrationsCredentialListMappingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a credential list mapping from the requested domain
pub async fn delete_sip_auth_registrations_credential_list_mapping(configuration: &configuration::Configuration, params: DeleteSipAuthRegistrationsCredentialListMappingParams) -> Result<(), Error<DeleteSipAuthRegistrationsCredentialListMappingError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let domain_sid = params.domain_sid;
    let sid = params.sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), DomainSid=crate::apis::urlencode(domain_sid), Sid=crate::apis::urlencode(sid));
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
        let local_var_entity: Option<DeleteSipAuthRegistrationsCredentialListMappingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch a specific instance of a credential list mapping
pub async fn fetch_sip_auth_registrations_credential_list_mapping(configuration: &configuration::Configuration, params: FetchSipAuthRegistrationsCredentialListMappingParams) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthRegistrationsPeriodSipAuthRegistrationsCredentialListMapping, Error<FetchSipAuthRegistrationsCredentialListMappingError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let domain_sid = params.domain_sid;
    let sid = params.sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), DomainSid=crate::apis::urlencode(domain_sid), Sid=crate::apis::urlencode(sid));
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
        let local_var_entity: Option<FetchSipAuthRegistrationsCredentialListMappingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a list of credential list mappings belonging to the domain used in the request
pub async fn list_sip_auth_registrations_credential_list_mapping(configuration: &configuration::Configuration, params: ListSipAuthRegistrationsCredentialListMappingParams) -> Result<crate::models::ListSipAuthRegistrationsCredentialListMappingResponse, Error<ListSipAuthRegistrationsCredentialListMappingError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let domain_sid = params.domain_sid;
    let page_size = params.page_size;
    let page = params.page;
    let page_token = params.page_token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), DomainSid=crate::apis::urlencode(domain_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<ListSipAuthRegistrationsCredentialListMappingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

