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
pub enum IncomingPhoneNumberLocalEnumEmergencyAddressStatus {
    #[serde(rename = "registered")]
    Registered,
    #[serde(rename = "unregistered")]
    Unregistered,
    #[serde(rename = "pending-registration")]
    PendingRegistration,
    #[serde(rename = "registration-failure")]
    RegistrationFailure,
    #[serde(rename = "pending-unregistration")]
    PendingUnregistration,
    #[serde(rename = "unregistration-failure")]
    UnregistrationFailure,

}

impl ToString for IncomingPhoneNumberLocalEnumEmergencyAddressStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Registered => String::from("registered"),
            Self::Unregistered => String::from("unregistered"),
            Self::PendingRegistration => String::from("pending-registration"),
            Self::RegistrationFailure => String::from("registration-failure"),
            Self::PendingUnregistration => String::from("pending-unregistration"),
            Self::UnregistrationFailure => String::from("unregistration-failure"),
        }
    }
}

impl Default for IncomingPhoneNumberLocalEnumEmergencyAddressStatus {
    fn default() -> IncomingPhoneNumberLocalEnumEmergencyAddressStatus {
        Self::Registered
    }
}



