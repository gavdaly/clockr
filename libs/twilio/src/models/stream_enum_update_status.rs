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
pub enum StreamEnumUpdateStatus {
    #[serde(rename = "stopped")]
    Stopped,
}

impl ToString for StreamEnumUpdateStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Stopped => String::from("stopped"),
        }
    }
}

impl Default for StreamEnumUpdateStatus {
    fn default() -> StreamEnumUpdateStatus {
        Self::Stopped
    }
}
