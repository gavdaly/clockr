ALTER TABLE magic_links
DROP CONSTRAINT IF EXISTS magic_links_user_id_key;

ALTER TABLE magic_links
ADD COLUMN IF NOT EXISTS purpose text NOT NULL DEFAULT 'sign_in',
ADD COLUMN IF NOT EXISTS expires_at timestamptz NOT NULL DEFAULT NOW() + INTERVAL '7 days',
ADD COLUMN IF NOT EXISTS consumed_at timestamptz;

CREATE INDEX IF NOT EXISTS magic_links_user_purpose_index
ON magic_links(user_id, purpose);

CREATE INDEX IF NOT EXISTS magic_links_available_index
ON magic_links(id, purpose, expires_at)
WHERE consumed_at IS NULL;
