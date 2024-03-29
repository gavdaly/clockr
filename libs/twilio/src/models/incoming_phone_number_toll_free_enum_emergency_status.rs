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
pub enum IncomingPhoneNumberTollFreeEnumEmergencyStatus {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Inactive")]
    Inactive,
}

impl ToString for IncomingPhoneNumberTollFreeEnumEmergencyStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Active => String::from("Active"),
            Self::Inactive => String::from("Inactive"),
        }
    }
}

impl Default for IncomingPhoneNumberTollFreeEnumEmergencyStatus {
    fn default() -> IncomingPhoneNumberTollFreeEnumEmergencyStatus {
        Self::Active
    }
}
