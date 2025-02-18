```sql

BEGIN;


CREATE TABLE IF NOT EXISTS time_log (
    id ULID NOT NULL DEFAULT ulid() PRIMARY KEY,  -- or uuid_generate_v4() depending on your extension
    user_id ULID NOT NULL REFERENCES users (id),
    event_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    location_latitude DECIMAL(9,6),           -- optional, for user geolocation
    location_longitude DECIMAL(9,6),          -- optional, for user geolocation
    mac_address MACADDR,                       -- the associated mac address for this time log entry
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),            -- timestamp of when this correction record was created
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()             -- timestamp of last update
);

CREATE TABLE IF NOT EXISTS time_log_correction (
    id ULID NOT NULL DEFAULT ulid() PRIMARY KEY,
    time_log_id ULID NOT NULL REFERENCES time_log (id),
    reason TEXT,                                             -- explanation for the correction
    state SMALLINT NOT NULL DEFAULT 0,                        -- integer representation of the correction state:
                                                              -- e.g., 0 = CREATED, 1 = APPROVED, 2 = ACCEPTED
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),            -- timestamp of when this correction record was created
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()             -- timestamp of last update
);


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

```
