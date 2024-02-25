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
pub enum PaymentsEnumCapture {
    #[serde(rename = "payment-card-number")]
    PaymentCardNumber,
    #[serde(rename = "expiration-date")]
    ExpirationDate,
    #[serde(rename = "security-code")]
    SecurityCode,
    #[serde(rename = "postal-code")]
    PostalCode,
    #[serde(rename = "bank-routing-number")]
    BankRoutingNumber,
    #[serde(rename = "bank-account-number")]
    BankAccountNumber,
}

impl ToString for PaymentsEnumCapture {
    fn to_string(&self) -> String {
        match self {
            Self::PaymentCardNumber => String::from("payment-card-number"),
            Self::ExpirationDate => String::from("expiration-date"),
            Self::SecurityCode => String::from("security-code"),
            Self::PostalCode => String::from("postal-code"),
            Self::BankRoutingNumber => String::from("bank-routing-number"),
            Self::BankAccountNumber => String::from("bank-account-number"),
        }
    }
}

impl Default for PaymentsEnumCapture {
    fn default() -> PaymentsEnumCapture {
        Self::PaymentCardNumber
    }
}
