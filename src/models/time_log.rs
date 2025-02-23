
#[derive(Clone, Debug)]
pub struct TimeLog {
    pub id: String,
    user_id: String,
    pub event_time: i64,
    location_latitude: Option<f64>,
    location_longitude: Option<f64>,
    mac_address: Option<String>,
    correction: Option<Correction>
}

#[derive(Clone, Debug)]
pub struct Correction {
    id: String,
    reason: String,
    state: CorrectionState,
}

#[derive(Clone, Debug, Default)]
pub enum CorrectionState {
    #[default]
    Pending = 0,
    Accepted = 1,
    Rejected = 2,
}

impl TimeLog {
    pub fn entry(&self) -> (String, i64) {
        ("8:03 AM".to_string(), self.event_time)
    }
    pub fn new(user_id: String, event_time: i64) -> Self {
        Self {
            id: "".to_string(),
            user_id,
            event_time,
            location_latitude: None,
            location_longitude: None,
            mac_address: None,
            correction: None,
        }
    }
}