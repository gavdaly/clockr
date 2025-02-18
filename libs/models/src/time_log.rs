use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct TimeLog {
    id: Ulid,
    pub event_time: Option<chrono::DateTime<chrono::Utc>>,
    pub location_latitude: Option<f64>,
    pub location_longitude: Option<f64>,
    pub ip_address: Option<std::net::IpAddr>,
    pub correction_reason: Option<String>,
    pub correction_state: Option<i16>, // 0=CREATED, 1=APPROVED, 2=ACCEPTED, etc.
}
