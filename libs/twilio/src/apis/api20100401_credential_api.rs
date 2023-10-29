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

/// struct for passing parameters to the method [`create_sip_credential`]
#[derive(Clone, Debug)]
pub struct CreateSipCredentialParams {
    /// The unique id of the Account that is responsible for this resource.
    pub account_sid: String,
    /// The unique id that identifies the credential list to include the created credential.
    pub credential_list_sid: String,
    /// The username that will be passed when authenticating SIP requests. The username should be sent in response to Twilio's challenge of the initial INVITE. It can be up to 32 characters long.
    pub username: String,
    /// The password that the username will use when authenticating SIP requests. The password must be a minimum of 12 characters, contain at least 1 digit, and have mixed case. (eg `IWasAtSignal2018`)
    pub password: String
}

/// struct for passing parameters to the method [`delete_sip_credential`]
#[derive(Clone, Debug)]
pub struct DeleteSipCredentialParams {
    /// The unique id of the Account that is responsible for this resource.
    pub account_sid: String,
    /// The unique id that identifies the credential list that contains the desired credentials.
    pub credential_list_sid: String,
    /// The unique id that identifies the resource to delete.
    pub sid: String
}

/// struct for passing parameters to the method [`fetch_sip_credential`]
#[derive(Clone, Debug)]
pub struct FetchSipCredentialParams {
    /// The unique id of the Account that is responsible for this resource.
    pub account_sid: String,
    /// The unique id that identifies the credential list that contains the desired credential.
    pub credential_list_sid: String,
    /// The unique id that identifies the resource to fetch.
    pub sid: String
}

/// struct for passing parameters to the method [`list_sip_credential`]
#[derive(Clone, Debug)]
pub struct ListSipCredentialParams {
    /// The unique id of the Account that is responsible for this resource.
    pub account_sid: String,
    /// The unique id that identifies the credential list that contains the desired credentials.
    pub credential_list_sid: String,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>
}

/// struct for passing parameters to the method [`update_sip_credential`]
#[derive(Clone, Debug)]
pub struct UpdateSipCredentialParams {
    /// The unique id of the Account that is responsible for this resource.
    pub account_sid: String,
    /// The unique id that identifies the credential list that includes this credential.
    pub credential_list_sid: String,
    /// The unique id that identifies the resource to update.
    pub sid: String,
    /// The password that the username will use when authenticating SIP requests. The password must be a minimum of 12 characters, contain at least 1 digit, and have mixed case. (eg `IWasAtSignal2018`)
    pub password: Option<String>
}


/// struct for typed errors of method [`create_sip_credential`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSipCredentialError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_sip_credential`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSipCredentialError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_sip_credential`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchSipCredentialError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_sip_credential`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSipCredentialError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_sip_credential`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSipCredentialError {
    UnknownValue(serde_json::Value),
}


/// Create a new credential resource.
pub async fn create_sip_credential(configuration: &configuration::Configuration, params: CreateSipCredentialParams) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialListPeriodSipCredential, Error<CreateSipCredentialError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let credential_list_sid = params.credential_list_sid;
    let username = params.username;
    let password = params.password;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), CredentialListSid=crate::apis::urlencode(credential_list_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("Username", username.to_string());
    local_var_form_params.insert("Password", password.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateSipCredentialError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a credential resource.
pub async fn delete_sip_credential(configuration: &configuration::Configuration, params: DeleteSipCredentialParams) -> Result<(), Error<DeleteSipCredentialError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let credential_list_sid = params.credential_list_sid;
    let sid = params.sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), CredentialListSid=crate::apis::urlencode(credential_list_sid), Sid=crate::apis::urlencode(sid));
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
        let local_var_entity: Option<DeleteSipCredentialError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch a single credential.
pub async fn fetch_sip_credential(configuration: &configuration::Configuration, params: FetchSipCredentialParams) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialListPeriodSipCredential, Error<FetchSipCredentialError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let credential_list_sid = params.credential_list_sid;
    let sid = params.sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), CredentialListSid=crate::apis::urlencode(credential_list_sid), Sid=crate::apis::urlencode(sid));
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
        let local_var_entity: Option<FetchSipCredentialError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a list of credentials.
pub async fn list_sip_credential(configuration: &configuration::Configuration, params: ListSipCredentialParams) -> Result<crate::models::ListSipCredentialResponse, Error<ListSipCredentialError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let credential_list_sid = params.credential_list_sid;
    let page_size = params.page_size;
    let page = params.page;
    let page_token = params.page_token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), CredentialListSid=crate::apis::urlencode(credential_list_sid));
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
        let local_var_entity: Option<ListSipCredentialError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a credential resource.
pub async fn update_sip_credential(configuration: &configuration::Configuration, params: UpdateSipCredentialParams) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialListPeriodSipCredential, Error<UpdateSipCredentialError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let credential_list_sid = params.credential_list_sid;
    let sid = params.sid;
    let password = params.password;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), CredentialListSid=crate::apis::urlencode(credential_list_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = password {
        local_var_form_params.insert("Password", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateSipCredentialError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

