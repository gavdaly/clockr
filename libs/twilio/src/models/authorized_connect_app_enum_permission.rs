/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.51.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthorizedConnectAppEnumPermission {
    #[serde(rename = "get-all")]
    GetAll,
    #[serde(rename = "post-all")]
    PostAll,
}

impl ToString for AuthorizedConnectAppEnumPermission {
    fn to_string(&self) -> String {
        match self {
            Self::GetAll => String::from("get-all"),
            Self::PostAll => String::from("post-all"),
        }
    }
}

impl Default for AuthorizedConnectAppEnumPermission {
    fn default() -> AuthorizedConnectAppEnumPermission {
        Self::GetAll
    }
}
