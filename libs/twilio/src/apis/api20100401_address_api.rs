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

/// struct for passing parameters to the method [`create_address`]
#[derive(Clone, Debug)]
pub struct CreateAddressParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will be responsible for the new Address resource.
    pub account_sid: String,
    /// The name to associate with the new address.
    pub customer_name: String,
    /// The number and street address of the new address.
    pub street: String,
    /// The city of the new address.
    pub city: String,
    /// The state or region of the new address.
    pub region: String,
    /// The postal code of the new address.
    pub postal_code: String,
    /// The ISO country code of the new address.
    pub iso_country: String,
    /// A descriptive string that you create to describe the new address. It can be up to 64 characters long.
    pub friendly_name: Option<String>,
    /// Whether to enable emergency calling on the new address. Can be: `true` or `false`.
    pub emergency_enabled: Option<bool>,
    /// Whether we should automatically correct the address. Can be: `true` or `false` and the default is `true`. If empty or `true`, we will correct the address you provide if necessary. If `false`, we won't alter the address you provide.
    pub auto_correct_address: Option<bool>,
    /// The additional number and street address of the address.
    pub street_secondary: Option<String>
}

/// struct for passing parameters to the method [`delete_address`]
#[derive(Clone, Debug)]
pub struct DeleteAddressParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that is responsible for the Address resource to delete.
    pub account_sid: String,
    /// The Twilio-provided string that uniquely identifies the Address resource to delete.
    pub sid: String
}

/// struct for passing parameters to the method [`fetch_address`]
#[derive(Clone, Debug)]
pub struct FetchAddressParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that is responsible for the Address resource to fetch.
    pub account_sid: String,
    /// The Twilio-provided string that uniquely identifies the Address resource to fetch.
    pub sid: String
}

/// struct for passing parameters to the method [`list_address`]
#[derive(Clone, Debug)]
pub struct ListAddressParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that is responsible for the Address resource to read.
    pub account_sid: String,
    /// The `customer_name` of the Address resources to read.
    pub customer_name: Option<String>,
    /// The string that identifies the Address resources to read.
    pub friendly_name: Option<String>,
    /// The ISO country code of the Address resources to read.
    pub iso_country: Option<String>,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>
}

/// struct for passing parameters to the method [`update_address`]
#[derive(Clone, Debug)]
pub struct UpdateAddressParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that is responsible for the Address resource to update.
    pub account_sid: String,
    /// The Twilio-provided string that uniquely identifies the Address resource to update.
    pub sid: String,
    /// A descriptive string that you create to describe the address. It can be up to 64 characters long.
    pub friendly_name: Option<String>,
    /// The name to associate with the address.
    pub customer_name: Option<String>,
    /// The number and street address of the address.
    pub street: Option<String>,
    /// The city of the address.
    pub city: Option<String>,
    /// The state or region of the address.
    pub region: Option<String>,
    /// The postal code of the address.
    pub postal_code: Option<String>,
    /// Whether to enable emergency calling on the address. Can be: `true` or `false`.
    pub emergency_enabled: Option<bool>,
    /// Whether we should automatically correct the address. Can be: `true` or `false` and the default is `true`. If empty or `true`, we will correct the address you provide if necessary. If `false`, we won't alter the address you provide.
    pub auto_correct_address: Option<bool>,
    /// The additional number and street address of the address.
    pub street_secondary: Option<String>
}


/// struct for typed errors of method [`create_address`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAddressError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_address`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAddressError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_address`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchAddressError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_address`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAddressError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_address`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAddressError {
    UnknownValue(serde_json::Value),
}


/// 
pub async fn create_address(configuration: &configuration::Configuration, params: CreateAddressParams) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodAddress, Error<CreateAddressError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let customer_name = params.customer_name;
    let street = params.street;
    let city = params.city;
    let region = params.region;
    let postal_code = params.postal_code;
    let iso_country = params.iso_country;
    let friendly_name = params.friendly_name;
    let emergency_enabled = params.emergency_enabled;
    let auto_correct_address = params.auto_correct_address;
    let street_secondary = params.street_secondary;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Addresses.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("CustomerName", customer_name.to_string());
    local_var_form_params.insert("Street", street.to_string());
    local_var_form_params.insert("City", city.to_string());
    local_var_form_params.insert("Region", region.to_string());
    local_var_form_params.insert("PostalCode", postal_code.to_string());
    local_var_form_params.insert("IsoCountry", iso_country.to_string());
    if let Some(local_var_param_value) = friendly_name {
        local_var_form_params.insert("FriendlyName", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = emergency_enabled {
        local_var_form_params.insert("EmergencyEnabled", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = auto_correct_address {
        local_var_form_params.insert("AutoCorrectAddress", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = street_secondary {
        local_var_form_params.insert("StreetSecondary", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 
pub async fn delete_address(configuration: &configuration::Configuration, params: DeleteAddressParams) -> Result<(), Error<DeleteAddressError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let sid = params.sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Addresses/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), Sid=crate::apis::urlencode(sid));
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
        let local_var_entity: Option<DeleteAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 
pub async fn fetch_address(configuration: &configuration::Configuration, params: FetchAddressParams) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodAddress, Error<FetchAddressError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let sid = params.sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Addresses/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), Sid=crate::apis::urlencode(sid));
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
        let local_var_entity: Option<FetchAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 
pub async fn list_address(configuration: &configuration::Configuration, params: ListAddressParams) -> Result<crate::models::ListAddressResponse, Error<ListAddressError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let customer_name = params.customer_name;
    let friendly_name = params.friendly_name;
    let iso_country = params.iso_country;
    let page_size = params.page_size;
    let page = params.page;
    let page_token = params.page_token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Addresses.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = customer_name {
        local_var_req_builder = local_var_req_builder.query(&[("CustomerName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = friendly_name {
        local_var_req_builder = local_var_req_builder.query(&[("FriendlyName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = iso_country {
        local_var_req_builder = local_var_req_builder.query(&[("IsoCountry", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 
pub async fn update_address(configuration: &configuration::Configuration, params: UpdateAddressParams) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodAddress, Error<UpdateAddressError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let sid = params.sid;
    let friendly_name = params.friendly_name;
    let customer_name = params.customer_name;
    let street = params.street;
    let city = params.city;
    let region = params.region;
    let postal_code = params.postal_code;
    let emergency_enabled = params.emergency_enabled;
    let auto_correct_address = params.auto_correct_address;
    let street_secondary = params.street_secondary;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Addresses/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), Sid=crate::apis::urlencode(sid));
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
    if let Some(local_var_param_value) = customer_name {
        local_var_form_params.insert("CustomerName", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = street {
        local_var_form_params.insert("Street", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = city {
        local_var_form_params.insert("City", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = region {
        local_var_form_params.insert("Region", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = postal_code {
        local_var_form_params.insert("PostalCode", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = emergency_enabled {
        local_var_form_params.insert("EmergencyEnabled", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = auto_correct_address {
        local_var_form_params.insert("AutoCorrectAddress", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = street_secondary {
        local_var_form_params.insert("StreetSecondary", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

