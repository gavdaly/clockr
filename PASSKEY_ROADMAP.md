# Passkey Roadmap

Passkeys are now the primary login path. SMS remains only as a temporary recovery and identity-proof path while existing users are migrated. Email is being introduced as the long-term recovery and enrollment channel, not as a routine passwordless login replacement.

## Current Auth Shape

- Primary login starts with a discoverable passkey prompt from the unauthenticated home screen.
- Transitional phone-hinted passkey login still exists in `src/screens/authenticate.rs`.
- SMS PIN recovery still exists in `src/screens/authenticate.rs`, `src/app.rs`, and `src/service/sms.rs`.
- Email addresses are stored on `users.email`, with `users.email_verified_at` tracking verified recovery addresses.
- Admins can create/edit users and create, copy, or send invite, recovery, and email-verification links from `/app/admin/users`.
- Magic links live in `magic_links` and now carry a `purpose`, optional bound `email`, expiry, and consumption state.
- Email sending is abstracted behind `src/service/email.rs`, currently with a log backend and disabled backend.
- Successful login stores the user id in `axum_session` under `id`.
- Passkey enforcement uses the `passkey_setup_required` session flag and `/app/passkeys/setup`.

## Target Flow

1. User signs in with a passkey as the normal path.
2. If the user has no connected account/passkey yet, an admin sends or copies an invite link.
3. Invite or recovery links create a session and force passkey enrollment when the account has no passkey.
4. Email verification proves the stored recovery email belongs to the user.
5. Email becomes the only recovery path after the migration window.
6. SMS/Twilio is removed after users have passkeys and verified recovery email.

## Completed

- Added WebAuthn/passkey dependencies and browser wrapper code.
- Added passkey configuration through `WEBAUTHN_RP_ID`, `WEBAUTHN_RP_ORIGIN`, and `WEBAUTHN_RP_NAME`.
- Added `user_passkeys` and `webauthn_challenges` tables.
- Added passkey DB helpers in `src/models/passkey.rs`.
- Added WebAuthn setup in `src/service/passkeys.rs`.
- Added passkey registration server functions.
- Added browser `navigator.credentials.create()` and `navigator.credentials.get()` wrappers.
- Added passkey enrollment UI and forced setup route.
- Added phone-hinted transitional passkey login.
- Added discoverable primary passkey login.
- Made the unauthenticated login screen passkey-first.
- Kept SMS recovery visible only as a transitional fallback.
- Added `users.email`.
- Added `users.email_verified_at`.
- Added purpose-aware, expiring, consumable magic links.
- Added email-bound magic links for invite, recovery, and verification.
- Added admin user list/create/edit views.
- Added admin copy-link actions for invite and recovery links.
- Added admin send-email actions for invite, recovery, and verification links.
- Added email-sending abstraction in `src/service/email.rs`.
- Obfuscated emails in tracing instead of skipping them entirely.
- Prevented email recovery sends unless the user's email is verified.
- Consuming invite or recovery links verifies the bound email when present.
- Added `/email/verify/:link` for explicit email verification.
- Ran SQLx-backed clippy clean with `-D warnings` for the current implementation.

## Remaining Work

### 1. Real Email Delivery

The email service abstraction exists, but it only logs or disables delivery.

- Choose a provider or SMTP strategy.
- Add provider-specific config.
- Implement a real `EMAIL_BACKEND`.
- Keep traces free of full magic links and full email addresses.
- Add provider failure handling with clear admin-facing errors.

### 2. User-Facing Email Management

Admins can store and verify email for users, but signed-in users cannot manage their own recovery email yet.

- Add a user settings/security screen.
- Let signed-in users add or change their email.
- Send verification when email changes.
- Show verified/unverified state.
- Consider requiring recent passkey auth before changing recovery email.

### 3. Recovery UX Polish

The recovery mechanics are present, but the UX can be clearer.

- Disable or explain recovery email actions for unverified emails in the admin UI.
- Show better messages when a recovery email cannot be sent.
- Decide whether admins should be able to resend verification from the edit-user screen as well as the list.
- Add clearer copy for invite versus recovery links.

### 4. Passkey Management

Users can add a passkey, but full management is still thin.

- Show a list of enrolled passkeys.
- Show last-used/device metadata.
- Allow renaming passkeys.
- Allow deleting passkeys after recent passkey auth or recovery verification.
- Prevent deleting the last usable sign-in method.

### 5. Hardening

Before treating this as production-ready auth:

- Rate-limit passkey start/finish functions.
- Rate-limit magic-link creation and consumption.
- Add cleanup for expired WebAuthn challenges and magic links.
- Add audit-event persistence for invite, recovery, verification, and passkey changes.
- Keep logging credential payloads, WebAuthn ceremony state, full links, and full emails out of traces.
- Require verified email before disabling SMS for any account.
- Add an explicit unsupported-browser recovery path.

### 6. Tests

Add coverage for:

- Challenge insert/load/consume behavior.
- Expired challenge rejection.
- Consumed challenge rejection.
- User mismatch rejection during registration finish.
- Session creation after successful passkey login.
- Magic-link purpose handling.
- Magic-link expiry and single-use behavior.
- Email-bound verification behavior.
- Recovery email rejection when email is missing or unverified.
- Admin-only magic-link creation and sending.

### 7. Manual Verification

Before deploy, manually verify:

- Enroll a passkey after SMS recovery on localhost.
- Enroll a passkey after invite-link login.
- Log out and sign in with the primary passkey button.
- Log in with phone-hinted passkey from the transitional screen.
- Verify an email from `/email/verify/:link`.
- Recover by verified email and enroll a replacement passkey.
- Cancel browser passkey prompts and confirm UI recovers.
- Try finishing the same WebAuthn challenge twice and confirm the second attempt fails.
- Try consuming the same magic link twice and confirm the second attempt fails.
- Confirm production config uses HTTPS origin and the correct RP ID.

## Twilio Removal Plan

Do not remove Twilio yet. The migration window is expected to last a couple of months while users get passkeys and verified recovery email.

Removal checklist:

- Every active user has at least one passkey.
- Every active user has a verified recovery email or another approved recovery method.
- Admin invite and recovery email delivery is backed by a real provider.
- SMS recovery usage has dropped to zero or an accepted operational threshold.
- Stop creating new SMS PIN challenges.
- Remove SMS UI from unauthenticated and recovery flows.
- Remove Twilio config and generated client code.
- Remove `src/service/sms.rs`.
- Remove phone-number login/recovery once operationally safe.

## Later Improvements

- Multi-tenant account model.
- Tenant-scoped admin users screen and invite links.
- Recovery codes so recovery does not depend entirely on email.
- Step-up auth for sensitive admin and account actions.
- Username-less passkey autofill hints if the UX benefits from it.
- Better device/browser names for passkeys.
