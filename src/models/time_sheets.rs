use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use {
    chrono::{Datelike, Duration, NaiveDate},
    uuid::Uuid,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TimeSheet {
    pub user_id: String,
    pub weeks: Vec<Week>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Week {
    pub start_date: String,
    pub end_date: String,
    pub days: Vec<Day>,
    pub total_hours: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Day {
    pub date: String,
    pub entries: Vec<TimeEntry>,
    pub total_hours: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TimeEntry {
    pub id: String,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub duration: Option<i64>,
    pub correction: Option<super::Correction>,
}

#[cfg(feature = "ssr")]
impl TimeSheet {
    pub async fn generate_for(
        user_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Self, sqlx::Error> {
        use crate::database::get_db;
        use crate::models::{CorrectionDB, TimeLog};
        use std::collections::HashMap;

        let db = get_db();

        // Fetch all time logs for the user within the date range
        let logs = sqlx::query!(
            r#"
            SELECT
                tl.id as time_log_id,
                tl.event_time,
                c.reason as "correction_reason?",
                c.state as "correction_state?"
            FROM time_log tl
            LEFT JOIN time_log_correction c ON c.time_log_id = tl.id
            WHERE tl.user_id = $1
            AND tl.event_time >= $2
            AND tl.event_time <= $3
            ORDER BY tl.event_time ASC
            "#,
            user_id,
            start_date.and_hms_opt(0, 0, 0).unwrap().and_utc(),
            end_date.and_hms_opt(23, 59, 59).unwrap().and_utc()
        )
        .fetch_all(db)
        .await?;

        let mut logs_by_date: HashMap<String, Vec<TimeLog>> = HashMap::new();

        for log in logs {
            let event_time = log.event_time;
            let date = event_time.format("%Y-%m-%d").to_string();

            let correction = CorrectionDB::from_options(
                log.correction_reason,
                log.correction_state,
                Some(ulid::Ulid::from_string(&log.time_log_id.to_string()).unwrap()),
            );

            let time_log = TimeLog {
                id: log.time_log_id.to_string(),
                event_time: event_time.timestamp(),
                correction: correction.map(|c| c.into()),
            };

            logs_by_date
                .entry(date).or_default()
                .push(time_log);
        }

        // Generate weeks and days
        let mut current_date = start_date;
        let mut weeks: Vec<Week> = Vec::new();
        let mut current_week: Option<Week> = None;

        while current_date <= end_date {
            let date_str = current_date.format("%Y-%m-%d").to_string();

            let weekday = current_date.weekday();
            if weekday == chrono::Weekday::Mon || current_week.is_none() {
                if let Some(week) = current_week {
                    weeks.push(week);
                }

                let week_end = current_date + Duration::days(6);
                let week_end_str = week_end.format("%Y-%m-%d").to_string();

                current_week = Some(Week {
                    start_date: date_str.clone(),
                    end_date: week_end_str,
                    days: Vec::new(),
                    total_hours: 0.0,
                });
            }

            let day_logs = logs_by_date.get(&date_str).cloned().unwrap_or_default();

            let mut entries: Vec<TimeEntry> = Vec::new();
            let mut day_total_seconds = 0;

            for chunk in day_logs.chunks(2) {
                let start_time = chunk[0].event_time;
                let end_time = if chunk.len() > 1 {
                    Some(chunk[1].event_time)
                } else {
                    None
                };

                let duration = end_time.map(|end| end - start_time);
                if let Some(dur) = duration {
                    day_total_seconds += dur;
                }

                entries.push(TimeEntry {
                    id: chunk[0].id.clone(),
                    start_time,
                    end_time,
                    duration,
                    correction: chunk[0].correction.clone(),
                });
            }

            // Calculate day total hours
            let day_total_hours = day_total_seconds as f64 / 3600.0;

            // Create the day
            let day = Day {
                date: date_str,
                entries,
                total_hours: day_total_hours,
            };

            // Add the day to the current week
            if let Some(week) = &mut current_week {
                week.total_hours += day_total_hours;
                week.days.push(day);
            }

            // Move to the next day
            current_date += Duration::days(1);
        }

        // Add the last week
        if let Some(week) = current_week {
            weeks.push(week);
        }

        Ok(Self {
            user_id: user_id.to_string(),
            weeks,
        })
    }
}

