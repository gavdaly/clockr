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

/// struct for passing parameters to the method [`fetch_member`]
#[derive(Clone, Debug)]
pub struct FetchMemberParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Member resource(s) to fetch.
    pub account_sid: String,
    /// The SID of the Queue in which to find the members to fetch.
    pub queue_sid: String,
    /// The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resource(s) to fetch.
    pub call_sid: String,
}

/// struct for passing parameters to the method [`list_member`]
#[derive(Clone, Debug)]
pub struct ListMemberParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Member resource(s) to read.
    pub account_sid: String,
    /// The SID of the Queue in which to find the members
    pub queue_sid: String,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>,
}

/// struct for passing parameters to the method [`update_member`]
#[derive(Clone, Debug)]
pub struct UpdateMemberParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Member resource(s) to update.
    pub account_sid: String,
    /// The SID of the Queue in which to find the members to update.
    pub queue_sid: String,
    /// The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resource(s) to update.
    pub call_sid: String,
    /// The absolute URL of the Queue resource.
    pub url: String,
    /// How to pass the update request data. Can be `GET` or `POST` and the default is `POST`. `POST` sends the data as encoded form data and `GET` sends the data as query parameters.
    pub method: Option<String>,
}

/// struct for typed errors of method [`fetch_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchMemberError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMemberError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMemberError {
    UnknownValue(serde_json::Value),
}

/// Fetch a specific member from the queue
pub async fn fetch_member(
    configuration: &configuration::Configuration,
    params: FetchMemberParams,
) -> Result<
    crate::models::ApiPeriodV2010PeriodAccountPeriodQueuePeriodMember,
    Error<FetchMemberError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let queue_sid = params.queue_sid;
    let call_sid = params.call_sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members/{CallSid}.json",
        local_var_configuration.base_path,
        AccountSid = crate::apis::urlencode(account_sid),
        QueueSid = crate::apis::urlencode(queue_sid),
        CallSid = crate::apis::urlencode(call_sid)
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
        let local_var_entity: Option<FetchMemberError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the members of the queue
pub async fn list_member(
    configuration: &configuration::Configuration,
    params: ListMemberParams,
) -> Result<crate::models::ListMemberResponse, Error<ListMemberError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let queue_sid = params.queue_sid;
    let page_size = params.page_size;
    let page = params.page;
    let page_token = params.page_token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members.json",
        local_var_configuration.base_path,
        AccountSid = crate::apis::urlencode(account_sid),
        QueueSid = crate::apis::urlencode(queue_sid)
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
        let local_var_entity: Option<ListMemberError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Dequeue a member from a queue and have the member's call begin executing the TwiML document at that URL
pub async fn update_member(
    configuration: &configuration::Configuration,
    params: UpdateMemberParams,
) -> Result<
    crate::models::ApiPeriodV2010PeriodAccountPeriodQueuePeriodMember,
    Error<UpdateMemberError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let queue_sid = params.queue_sid;
    let call_sid = params.call_sid;
    let url = params.url;
    let method = params.method;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members/{CallSid}.json",
        local_var_configuration.base_path,
        AccountSid = crate::apis::urlencode(account_sid),
        QueueSid = crate::apis::urlencode(queue_sid),
        CallSid = crate::apis::urlencode(call_sid)
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
    local_var_form_params.insert("Url", url.to_string());
    if let Some(local_var_param_value) = method {
        local_var_form_params.insert("Method", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateMemberError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
