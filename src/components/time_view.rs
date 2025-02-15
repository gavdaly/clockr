use chrono::{Datelike, Duration, NaiveDate, TimeZone, Utc};
use leptos::*;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/*

BEGIN;

-- 1. Enable the UUID generation extension if needed
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- or if you prefer the uuid-ossp extension:
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 2. Create the new time_log table
CREATE TABLE IF NOT EXISTS time_log (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,  -- or uuid_generate_v4() depending on your extension
    user_id UUID NOT NULL REFERENCES users (id),
    event_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    location_latitude DECIMAL(9,6),           -- optional, for user geolocation
    location_longitude DECIMAL(9,6),          -- optional, for user geolocation
    ip_address INET                           -- optional, for storing IPv4 or IPv6 address
);

-- 3. Create a separate table for corrections on non-manual entries
CREATE TABLE IF NOT EXISTS time_log_correction (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    time_log_id UUID NOT NULL REFERENCES time_log (id),
    reason TEXT,                                             -- explanation for the correction
    state SMALLINT NOT NULL DEFAULT 0,                        -- integer representation of the correction state:
                                                              -- e.g., 0 = CREATED, 1 = APPROVED, 2 = ACCEPTED
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),            -- timestamp of when this correction record was created
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()             -- timestamp of last update
);

-- 4. Migrate existing data from sessions to time_log
INSERT INTO time_log (user_id, event_time)
SELECT user_id, start_time
FROM sessions
WHERE start_time IS NOT NULL;

INSERT INTO time_log (user_id, event_time)
SELECT user_id, end_time
FROM sessions
WHERE end_time IS NOT NULL;

-- 5. (Optional) Drop the old sessions table if no longer needed
-- DROP TABLE sessions;

-- 6. Create or replace a view that joins users, time_log, and time_log_correction
-- so you can easily filter by user_id and a specific date.
CREATE OR REPLACE VIEW v_user_time_overview AS
SELECT
    u.id as user_id,
    u.name AS user_name,                -- adjust field names as per your 'users' table
    t.id AS time_log_id,
    t.event_time,
    t.location_latitude,
    t.location_longitude,
    t.ip_address,
    c.id AS correction_id,
    c.reason AS correction_reason,
    c.state AS correction_state,
    c.created_at AS correction_created_at,
    c.updated_at AS correction_updated_at
FROM users u
JOIN time_log t ON t.user_id = u.id
LEFT JOIN time_log_correction c ON c.time_log_id = t.id;

COMMIT;






*/

// 1. Define a data structure representing a single row from the v_user_time_overview.
//    Adjust field names/types to match your actual columns.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserTimeOverview {
    pub user_id: Uuid,
    pub user_name: String,
    pub time_log_id: Uuid,
    pub event_time: Option<chrono::DateTime<chrono::Utc>>,
    pub location_latitude: Option<f64>,
    pub location_longitude: Option<f64>,
    pub ip_address: Option<std::net::IpAddr>,
    pub correction_id: Option<Uuid>,
    pub correction_reason: Option<String>,
    pub correction_state: Option<i16>, // 0=CREATED, 1=APPROVED, 2=ACCEPTED, etc.
    pub correction_created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub correction_updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

// 2. Define a server function to fetch data from the view by user_id and an optional date range.
//    If none is provided, default to 2 weeks before the most recent Sunday up to now.
#[server(FetchUserTimeOverviewRange)]
pub async fn fetch_user_time_overview_range(
    user_id: Uuid,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
) -> Result<Vec<UserTimeOverview>, ServerFnError> {
    let now = Utc::now();
    let weekday = now.weekday().num_days_from_sunday(); // 0 = Sunday, 1 = Monday, etc.
    let recent_sunday = now.date_naive() - chrono::Duration::days(weekday.into());
    let two_weeks_ago_sunday = recent_sunday - chrono::Duration::days(14);

    let actual_from = from_date.unwrap_or(two_weeks_ago_sunday);
    let actual_to = to_date.unwrap_or(now.date_naive());

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

    let rows = sqlx::query_as::<_, UserTimeOverview>(query)
        .bind(user_id)
        .bind(actual_from)
        .bind(actual_to)
        .fetch_all(&pool)
        .await?;

    Ok(rows)
}

// 3. Define a Leptos component that uses fetch_user_time_overview_range to display data.
#[component]
pub fn UserTimeOverviewView(
    user_id: Uuid,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
) -> impl IntoView {
    // Use create_resource to call our server function.
    let fetch_data = create_resource(
        move || (user_id, from_date, to_date),
        move |(user_id, from_date, to_date)| {
            fetch_user_time_overview_range(user_id, from_date, to_date)
        },
    );

    view! {

        match fetch_data.read() {
            None => view! {  <div>"Loading..."</div> }.into_any(),
            Some(Err(e)) => view! {  <div class="text-red-600">{format!("Error: {}", e)}</div> }.into_any(),
            Some(Ok(rows)) => {
                if rows.is_empty() {
                    view! { <div>{"No records found."}</div> }.into_any()
                } else {
                    view! {
                        <table class="table-auto border-collapse w-full">
                            <thead>
                                <tr class="border-b">
                                    <th class="px-2 py-1 text-left">{"Event Time"}</th>
                                    <th class="px-2 py-1 text-left">{"Correction State"}</th>
                                    <th class="px-2 py-1 text-left">{"Reason"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {rows.into_iter().map(|row| {
                                    let event_time_str = row.event_time.map(|dt| dt.to_rfc3339()).unwrap_or_else(|| "N/A".to_string());

                                    let correction_state_str = row.correction_state.map(|s| s.to_string()).unwrap_or_else(|| "".to_string());
                                    let correction_reason = row.correction_reason.unwrap_or_default();

                                    view! {
                                        <tr class="border-b">
                                            <td class="px-2 py-1">{event_time_str}</td>
                                            <td class="px-2 py-1">{correction_state_str}</td>
                                            <td class="px-2 py-1">{correction_reason}</td>
                                        </tr>
                                    }
                                }).collect_view()}
                            </tbody>
                        </table>
                    }.into_any()
                }
            },
        }
    }
}

/*
USAGE EXAMPLE:

In your router or main app file:

#[component]
fn App() -> impl IntoView {
    let user_id = Uuid::parse_str("<some-user-uuid>").unwrap();

    // By default, we won't pass any date range. That means 2 weeks before recent Sunday until now.
    // If you want a custom date range, you can specify:
    // let from_date = Some(NaiveDate::from_ymd_opt(2025, 2, 1).unwrap());
    // let to_date = Some(NaiveDate::from_ymd_opt(2025, 2, 14).unwrap());

    view! {
        <main>
            <UserTimeOverviewView user_id=user_id from_date=None to_date=None />
        </main>
    }
}

*/
