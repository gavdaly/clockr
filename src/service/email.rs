#[cfg(feature = "ssr")]
#[derive(Clone, Copy, Debug)]
pub enum MagicLinkEmailKind {
    Invite,
    Recovery,
    Verification,
}

#[cfg(feature = "ssr")]
impl MagicLinkEmailKind {
    fn subject(self) -> &'static str {
        match self {
            Self::Invite => "Create your Clockr account",
            Self::Recovery => "Recover access to Clockr",
            Self::Verification => "Verify your Clockr email",
        }
    }

    fn body(self, link: &str) -> String {
        match self {
            Self::Invite => format!(
                "Use this link to create your Clockr account and set up a passkey:\n\n{link}\n\nThis link expires in 14 days."
            ),
            Self::Recovery => format!(
                "Use this link to recover access to Clockr and set up a passkey if needed:\n\n{link}\n\nThis link expires in 30 minutes."
            ),
            Self::Verification => format!(
                "Use this link to verify your email address for Clockr:\n\n{link}\n\nThis link expires in 24 hours."
            ),
        }
    }
}

#[cfg(feature = "ssr")]
#[tracing::instrument(skip(link), fields(to = %obfuscate_email(to), kind = ?kind))]
pub async fn send_magic_link(to: &str, link: &str, kind: MagicLinkEmailKind) -> crate::Result<()> {
    let sender = EmailSender::from_env();
    sender.send(to, kind.subject(), &kind.body(link)).await
}

#[cfg(feature = "ssr")]
struct EmailSender {
    backend: EmailBackend,
}

#[cfg(feature = "ssr")]
impl EmailSender {
    fn from_env() -> Self {
        let backend = match std::env::var("EMAIL_BACKEND") {
            Ok(value) if value.eq_ignore_ascii_case("disabled") => EmailBackend::Disabled,
            _ => EmailBackend::Log,
        };

        Self { backend }
    }

    async fn send(&self, to: &str, subject: &str, body: &str) -> crate::Result<()> {
        match self.backend {
            EmailBackend::Log => {
                tracing::info!(
                    to = %obfuscate_email(to),
                    subject,
                    body_bytes = body.len(),
                    "Email delivery is configured for log-only mode."
                );
                Ok(())
            }
            EmailBackend::Disabled => {
                tracing::warn!(
                    to = %obfuscate_email(to),
                    subject,
                    "Email delivery is disabled."
                );
                Err(crate::Error::InternalError)
            }
        }
    }
}

#[cfg(feature = "ssr")]
#[derive(Clone, Copy, Debug)]
enum EmailBackend {
    Log,
    Disabled,
}

#[cfg(feature = "ssr")]
fn obfuscate_email(email: &str) -> String {
    let Some((local, domain)) = email.split_once('@') else {
        return "[invalid-email]".to_string();
    };

    let visible_local: String = local.chars().take(1).collect();
    let visible_domain: String = domain.chars().take(1).collect();

    format!("{visible_local}***@{visible_domain}***")
}
