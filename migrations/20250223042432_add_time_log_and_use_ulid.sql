ALTER TABLE users 
    ADD COLUMN ulid UUID;

CREATE TABLE IF NOT EXISTS time_log (
    id UUID NOT NULL PRIMARY KEY, -- use uuid to store ulid, need to generate it on the server
    user_id UUID NOT NULL REFERENCES users (id),
    event_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    location_latitude DECIMAL(9,6),           
    location_longitude DECIMAL(9,6),          
    mac_address MACADDR,                      
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW() 
);

CREATE TABLE IF NOT EXISTS time_log_correction (
    id UUID NOT NULL PRIMARY KEY,
    time_log_id UUID NOT NULL REFERENCES time_log (id),
    reason TEXT,                                             
    state SMALLINT NOT NULL DEFAULT 0,                                                                                    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),           
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE VIEW v_user_time_overview AS
SELECT
    u.id,
    u.first_name,
    u.last_name,
    t.id AS time_log_id,
    t.event_time,
    t.location_latitude AS latitude,
    t.location_longitude AS longitude,
    c.id AS correction_id,
    c.reason AS correction_reason,
    c.state AS correction_state
FROM users u
JOIN time_log t ON t.user_id = u.id
LEFT JOIN time_log_correction c ON c.time_log_id = t.id;
