use chrono::{DateTime, Utc};
use ulid::Ulid;

struct InUserTransform {
    pub uuid: String,
    pub phone_number: String,
    pub created_at: DateTime<Utc>,
}

struct OutUserTransform {
    pub uuid: String,
    pub id: Ulid,
    pub contacts: Vec<String>,
}

pub fn transform_user(old: InUserTransform) -> OutUserTransform {
    let id = genenerate_ulid_from_date(old.created_at);
    OutUserTransform {
        uuid: old.uuid,
        id,
        contacts: vec![old.phone_number.to_string()],
    }
}

fn genenerate_ulid_from_date(date: DateTime<Utc>) -> Ulid {
    let timestamp = date.timestamp_millis() as u64;
    let rand: u128 = rand::random();
    Ulid::from_parts(timestamp, rand)
}

pub fn transform() {
    // get all current users
    let users = vec![];

    for user in users {
        let transformed_user = transform_user(user);
        // save transformed_user to database
        session_session_for_user(transformed_user);
    }
}

fn session_session_for_user(user: OutUserTransform) {
    // get sessions for user
    let sessions: Vec<Session> = vec![];

    for session in sessions {
        let convert = |time, user_id| {
            let id = genenerate_ulid_from_date(time);

            let query = "
                    INSERT INTO time_log (id, event_time, user_id)
                    VALUES ($1, $2, $3);";
        };
        let t = session.start_time;
        convert(t, user.id);

        if let Some(t2) = session.end_time {
            convert(t2, user.id);
        };
    }
}

#[derive(Clone, Debug)]
pub struct Session {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub state: i32,
    pub id: String,
    pub user_id: String,
}
