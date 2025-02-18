use super::{Result, TimeLog};
use chrono::{NaiveDate, Utc};
use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct UserOverview {
    id: Ulid,
    first_name: String,
    last_name: String,
    time_logs: Vec<TimeLog>,
}

pub async fn fetch_user_time_overview_range(
    user_id: Ulid,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
) -> Result<Vec<UserOverview>> {
    let now = Utc::now();
    // let weekday = now.date_naive().weekday().num_days_from_sunday();
    // let recent_sunday = now.date_naive() - chrono::Duration::days(weekday.into());
    // let two_weeks_ago_sunday = recent_sunday - chrono::Duration::days(14);

    let from = from_date.unwrap_or(now.date_naive());
    let to = to_date.unwrap_or(now.date_naive());

    let query = r#"
        SELECT
            user_id,
            user_name,
            time_log_id,
            event_time,
            location_latitude,
            location_longitude,
            ip_address,
            correction_id,
            correction_reason,
            correction_state,
            correction_created_at,
            correction_updated_at
        FROM v_user_time_overview
        WHERE user_id = $1
          AND event_time::date >= $2
          AND event_time::date <= $3
        ORDER BY event_time ASC
    "#;

    // let rows = sqlx::query_as::<_, UserTimeOverview>(query)
    //     .bind(user_id)
    //     .bind(from)
    //     .bind(to)
    //     .fetch_all()
    //     .await?;
    //
    Ok(vec![])
}
