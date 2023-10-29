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
pub enum SiprecEnumTrack {
    #[serde(rename = "inbound_track")]
    InboundTrack,
    #[serde(rename = "outbound_track")]
    OutboundTrack,
    #[serde(rename = "both_tracks")]
    BothTracks,

}

impl ToString for SiprecEnumTrack {
    fn to_string(&self) -> String {
        match self {
            Self::InboundTrack => String::from("inbound_track"),
            Self::OutboundTrack => String::from("outbound_track"),
            Self::BothTracks => String::from("both_tracks"),
        }
    }
}

impl Default for SiprecEnumTrack {
    fn default() -> SiprecEnumTrack {
        Self::InboundTrack
    }
}




