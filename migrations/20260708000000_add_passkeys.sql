CREATE TABLE IF NOT EXISTS user_passkeys (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    credential_id bytea NOT NULL UNIQUE,
    passkey jsonb NOT NULL,
    label text,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    last_used_at timestamptz
);

CREATE INDEX IF NOT EXISTS user_passkeys_user_id_idx
ON user_passkeys(user_id);

CREATE TABLE IF NOT EXISTS webauthn_challenges (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id uuid REFERENCES users(id) ON DELETE CASCADE,
    flow text NOT NULL CHECK (flow IN ('registration', 'authentication')),
    state jsonb NOT NULL,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    expires_at timestamptz NOT NULL,
    consumed_at timestamptz
);

CREATE INDEX IF NOT EXISTS webauthn_challenges_active_idx
ON webauthn_challenges(id, user_id, flow, expires_at, consumed_at);
