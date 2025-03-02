use super::TimeLog;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use {
    chrono::{DateTime, Local, NaiveDateTime, TimeZone, Weekday},
    std::collections::BTreeMap,
    ulid::Ulid,
};


#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum CurrentUser {
    Authenticated(UserToday),
    #[default]
    Guest,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserToday {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub state: i32,
    pub check_ins: Vec<TimeLog>,
    pub week_duration: i64,
    pub previous_day_possible_errors: i16,
}

#[cfg(feature = "ssr")]
pub struct UserTodayDB {
    id: Ulid,
    first_name: String,
    last_name: String,
    state: i32,
    time_log_id: Ulid,
    event_time: chrono::DateTime<chrono::Utc>,
    correction_reason: Option<String>,
    correction_state: Option<i16>,
}

#[cfg(feature = "ssr")]
impl UserTodayDB {
    #[tracing::instrument]
    pub async fn get(id: &str) -> Result<Vec<Self>, sqlx::Error> {
        use uuid::Uuid;

        let db = crate::database::get_db();
        let user_id = Uuid::parse_str(id).expect("Invalid UUID");

        let now = Local::now();
        let sunday = now
            .date_naive().week(Weekday::Sun)
            .first_day()
            .and_hms_opt(0, 0, 0)
            .expect("Invalid datetime");

        tracing::info!("Fetching overview for user {} from {}", id, sunday);

        sqlx::query_as!(
            UserTodayDB,
            r#"
            SELECT 
                u.id as id,
                u.first_name,
                u.last_name,
                u.state,
                tl.id as time_log_id,
                tl.event_time,
                c.reason as "correction_reason?",
                c.state as "correction_state?"
            FROM users u
            LEFT JOIN time_log tl ON tl.user_id = u.id
            LEFT JOIN time_log_correction c ON c.time_log_id = tl.id
            WHERE u.id = $1 
            AND tl.event_time >= $2
            ORDER BY tl.event_time ASC
            "#,
            user_id,
            sunday.and_utc()
        )
        .fetch_all(db)
        .await
    }
}

#[cfg(feature = "ssr")]
impl From<Vec<UserTodayDB>> for UserToday {
    fn from(overviews: Vec<UserTodayDB>) -> Self {
        use super::CorrectionDB;
        use uuid::Uuid;

        let Some(first) = overviews.first() else {
            return Self {
                id: String::new(),
                first_name: String::new(),
                last_name: String::new(),
                state: 0,
                check_ins: Vec::new(),
                week_duration: 0,
                previous_day_possible_errors: 0,
            };
        };

        // Group check-ins by date
        let mut logs_by_date: BTreeMap<String, Vec<TimeLog>> = BTreeMap::new();

        for overview in &overviews {
            let correction = CorrectionDB::from_options(
                overview.correction_reason.clone(),
                overview.correction_state,
                Some(overview.time_log_id.clone()),
            ); 
            let correction = match correction {
                Some(c) => Some(c.into()),
                None => None,
            };

            let time_log = TimeLog {
                id: Uuid::from_bytes(overview.time_log_id.to_bytes()).to_string(),
                event_time: overview.event_time.timestamp(),
                correction,
            };

            // Convert timestamp to date string
            let date = overview.event_time
                .format("%Y-%m-%d")
                .to_string();

            logs_by_date
                .entry(date)
                .or_insert_with(|| vec![])
                .push(time_log);
        }

        let today = Local::now().format("%Y-%m-%d").to_string();

        // Get today's check-ins
        let check_ins = logs_by_date.remove(&today).unwrap_or_default();

        // Calculate duration for previous days and count error days
        let mut week_duration = 0;
        let mut previous_day_possible_errors = 0;

        for (_date, logs) in logs_by_date {
            // Count days with odd number of logs
            if logs.len() % 2 == 1 {
                previous_day_possible_errors += 1;
            }

            // Calculate duration for completed pairs
            week_duration += logs
                .chunks(2)
                .filter(|chunk| chunk.len() == 2)
                .map(|chunk| chunk[1].event_time - chunk[0].event_time)
                .sum::<i64>();
        }

        Self {
            id: Uuid::from_bytes(first.id.to_bytes()).to_string(),
            first_name: first.first_name.clone(),
            last_name: first.last_name.clone(),
            state: first.state,
            check_ins,
            week_duration,
            previous_day_possible_errors,
        }
    }
}
