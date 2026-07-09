ALTER TABLE users
ADD COLUMN IF NOT EXISTS email_verified_at timestamptz;

ALTER TABLE magic_links
ADD COLUMN IF NOT EXISTS email text;

CREATE INDEX IF NOT EXISTS magic_links_email_verification_index
ON magic_links(user_id, email, purpose)
WHERE consumed_at IS NULL;
