# Passkey Roadmap

This roadmap makes passkeys the primary login method without depending on SMS long term. The first implementation can keep the existing SMS PIN flow as the temporary identity proof, but the target direction is passkey-first login with email used only for recovery, enrollment, and re-enrollment.

## Current Auth Shape

- SMS PIN login lives in `src/screens/authenticate.rs`.
- Twilio-backed SMS delivery is currently in `src/service/sms.rs`.
- Successful login stores the user id in `axum_session` under `id`.
- User lookup is currently by `users.phone_number`.
- PostgreSQL migrations live in `migrations/`.
- Shared DB access goes through `src/database.rs`.
- Leptos server functions are already used for auth, so passkey operations should also be server functions unless a later refactor moves auth to explicit Axum JSON routes.

## Target Flow

1. User proves identity with SMS PIN in the first pass, or an email recovery/enrollment link after the email migration.
2. Authenticated user clicks "Add passkey".
3. Server starts a WebAuthn registration ceremony and stores the ceremony state server-side.
4. Browser calls `navigator.credentials.create()`.
5. Server verifies the registration response and stores the passkey.
6. Future login uses passkeys as the primary path.
7. Email is used only when the user needs recovery, first-time enrollment, or re-enrollment on a new device.
8. SMS remains available only during the transition, then Twilio is removed.

Do not start with username-less autofill. Add that after the basic registration and login flows are stable.

## Identity Migration Strategy

Treat passkeys as the stable authentication layer. SMS and email are proof/recovery channels only; email should not become the default daily login path.

Short term:

- keep SMS PIN login working
- enroll passkeys only after a successful SMS PIN login
- store passkeys by `user_id`, not by phone number
- build server functions so passkey login does not depend on the user's phone number long term

Medium term:

- add `users.email` with a unique index, after a backfill and verification plan
- add an email verification table for short-lived recovery and enrollment links
- let users add and verify an email while signed in
- allow email recovery to create a temporary session that can enroll a new passkey
- keep SMS as fallback until enough users have verified email and passkeys

Long term:

- make passkeys the primary login surface
- make email the recovery channel, not the normal login method
- stop creating new SMS PIN challenges
- remove Twilio config, generated client code, and `src/service/sms.rs`
- remove phone-number login once operationally safe

## Phase 1: Dependencies And Config

Add server dependencies:

- `webauthn-rs` with `danger-allow-state-serialisation`
- `url`
- `serde_json` if not already sufficient for server-side JSON state

Add WASM/client dependencies:

- `wasm-bindgen-futures`
- `js-sys`
- `serde-wasm-bindgen`
- `base64ct`
- `web-sys` features for `Window`, `Navigator`, `CredentialsContainer`, `Credential`, and `PublicKeyCredential`

Add environment/config values:

- `WEBAUTHN_RP_ID`
- `WEBAUTHN_RP_ORIGIN`
- `WEBAUTHN_RP_NAME`

Local defaults should be:

- RP ID: `localhost`
- Origin: `http://localhost:3000`

Production must use the real HTTPS origin and registrable domain. WebAuthn is strict about origin and RP ID matching.

If email is added in the same broader auth project, add separate mailer config rather than coupling it to passkeys. Passkeys should only care about stable `user_id`; email/SMS should only prove that a user may recover access or enroll a credential.

## Phase 2: Database Schema

Add a migration for stored passkeys:

```sql
CREATE TABLE user_passkeys (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    credential_id bytea NOT NULL UNIQUE,
    passkey jsonb NOT NULL,
    label text,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    last_used_at timestamptz
);

CREATE INDEX user_passkeys_user_id_idx
ON user_passkeys(user_id);
```

Add a migration for WebAuthn challenge state:

```sql
CREATE TABLE webauthn_challenges (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id uuid REFERENCES users(id) ON DELETE CASCADE,
    flow text NOT NULL CHECK (flow IN ('registration', 'authentication')),
    state jsonb NOT NULL,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    expires_at timestamptz NOT NULL,
    consumed_at timestamptz
);

CREATE INDEX webauthn_challenges_active_idx
ON webauthn_challenges(id, user_id, flow, expires_at, consumed_at);
```

Rules:

- Expire challenge state after roughly five minutes.
- Mark a challenge consumed after successful verification.
- Never store serialized WebAuthn state in local storage, client-readable cookies, or hidden form fields.
- Tie registration challenges to the current session user.
- Tie authentication challenges to an allowed credential set. Initially this can be found from a phone-number hint; later prefer browser-discoverable credentials or an email hint only when recovering.

For the later email migration, add a separate migration:

```sql
ALTER TABLE users
ADD COLUMN email text;

CREATE UNIQUE INDEX users_email_unique_idx
ON users(lower(email))
WHERE email IS NOT NULL;
```

After email verification is implemented, add an `email_verified_at timestamptz` column or a separate verified-identities table. Do not treat an unverified email address as a recovery factor, and do not use email as a routine passwordless login replacement.

## Phase 3: Server Model Layer

Add a new module, likely `src/models/passkey.rs`, with DB helpers for:

- insert passkey
- list passkeys for a user
- delete passkey for a user
- find/update passkey by credential id
- insert challenge
- load unconsumed challenge by id, user, and flow
- consume challenge
- delete expired challenges

Keep WebAuthn serialized state as `serde_json::Value` in the DB layer and convert to the concrete `webauthn-rs` ceremony state types at the server-function boundary.

## Phase 4: Server WebAuthn Setup

Add a small server-only module, likely `src/service/passkeys.rs`, that builds a `Webauthn` instance from environment/config.

The current app uses global DB access rather than a broader custom `AppState`, so the first pass can use a server-only `OnceLock<Arc<Webauthn>>`, matching the existing `database.rs` style. A later app-state refactor can move DB and WebAuthn together.

Responsibilities:

- build `WebauthnBuilder` from RP ID, origin, and name
- expose a `get_webauthn()` helper
- fail loudly at startup or first use if config is invalid

## Phase 5: Registration Server Functions

Add `src/functions/passkeys.rs` and re-export it from `src/functions/mod.rs`.

Implement:

- `start_passkey_registration()`
- `finish_passkey_registration(challenge_id, credential_json)`

Registration requirements:

- User must already be logged in through the existing session.
- Use the current session user id from `current_user()`.
- Use the user's phone number or name fields as the WebAuthn user handle/display data.
- Store the registration state in `webauthn_challenges`.
- On finish, load only an unexpired, unconsumed registration challenge for the current user.
- Store the resulting passkey in `user_passkeys`.
- Mark the challenge consumed.

## Phase 6: Client Registration Wrapper

Add a client-side module, likely `src/app/passkey_client.rs`, for browser WebAuthn calls.

Implement:

- convert server-provided public key options into `JsValue`
- call `navigator.credentials.create()`
- convert `PublicKeyCredential` response ArrayBuffers into base64url JSON
- return JSON suitable for `finish_passkey_registration`

Keep this module small and isolated because WebAuthn browser response conversion is the most awkward part of the feature.

## Phase 7: Add Enrollment UI

Add an account/security area or a small section in the authenticated app with:

- "Add passkey" button
- list of existing passkeys
- delete passkey action
- clear error state for unsupported browser, failed verification, and cancelled browser prompt

The first implementation can use a simple label such as browser/device name entered after enrollment, or a default label like "Passkey".

## Phase 8: Login Server Functions

Implement:

- `start_passkey_login(phone: String)` for the transitional version
- `finish_passkey_login(challenge_id, credential_json)`

Login requirements:

- User enters phone number first in the transitional version if non-discoverable credentials are simpler to ship.
- Long term, prefer discoverable passkey login so the user can choose an account from the browser/OS passkey prompt.
- Email should not be required for normal passkey login.
- Server finds the user with `get_user_by_phone` initially if using a hint.
- Server loads passkeys for that user.
- Server starts an authentication ceremony and stores the state in `webauthn_challenges`.
- On finish, server verifies the assertion, updates `last_used_at`, consumes the challenge, and creates the normal `axum_session` session by setting `id`.

Return a clean error when the resolved user has no passkeys enrolled so the UI can direct the user to recovery/enrollment.

## Phase 9: Login UI

Update the unauthenticated flow with:

- "Sign in with passkey" as the primary action
- phone number input only for the transitional hinted passkey flow, if needed
- existing SMS login path only during the transition
- email recovery link for users who cannot use a passkey

Keep SMS visible during the transition. For the first pass, passkey login may require a phone number hint. After the passkey flow is solid, move toward discoverable passkey login instead of replacing the phone hint with an email hint.

## Phase 9a: Email Migration

Add email recovery before removing Twilio:

- add `email` and verified-email state to `users`
- create short-lived email verification and recovery tokens
- add a mail delivery service abstraction, for example `src/service/email.rs`
- add server functions for requesting and consuming email recovery links or codes
- add UI for signed-in users to add/verify email
- add UI for unauthenticated users to recover access by email
- after email recovery, create a limited session that requires passkey enrollment before normal app use unless the user is only managing recovery
- keep SMS fallback until existing users have a verified email or another recovery method

The important boundary: passkeys attach to `user_id`. Email recovery can restore access to that user and enroll a new passkey without turning email into the primary login method.

## Phase 10: Hardening

Before considering passkeys production-ready:

- rate-limit start and finish functions for both registration and login
- ensure challenges are single-use and short-lived
- add cleanup for expired challenges
- log failures without logging credential payloads or ceremony state
- require recent passkey auth or recovery verification before passkey deletion
- prevent users from deleting their last recovery method
- require verified email before disabling SMS for an account
- add a clear recovery path for unsupported browsers or unavailable passkeys

## Phase 11: Tests And Verification

Add coverage for:

- challenge insert/load/consume behavior
- expired challenge rejection
- consumed challenge rejection
- user mismatch rejection during registration finish
- passkey list/delete authorization
- session creation after successful passkey login

Manual browser verification:

- enroll a passkey after SMS login on localhost
- log out and sign in with phone plus passkey
- after discoverable login is added, sign in from the browser/OS passkey prompt without a phone or email hint
- after email migration, recover by email and enroll a replacement passkey
- cancel the browser prompt and confirm UI recovers
- try finishing the same challenge twice and confirm the second attempt fails
- verify production config uses HTTPS origin and the correct RP ID before deploy

## Later Improvements

- username-less passkey autofill
- passkey last-used/device metadata in the UI
- admin visibility for whether a user has enrolled a passkey
- step-up auth for sensitive actions
- recovery-code support so recovery does not depend entirely on email
- remove Twilio and phone-number login after email recovery is proven
