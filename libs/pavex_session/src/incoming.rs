use crate::{SessionId, State, config::SessionCookieConfig, wire::WireClientState};
use pavex::{cookie::RequestCookies, methods};
use pavex_tracing::fields::{ERROR_DETAILS, ERROR_MESSAGE, error_details, error_message};

/// The session information attached to the incoming request.
///
/// Built using [`IncomingSession::extract`].
pub struct IncomingSession {
    pub(crate) id: SessionId,
    pub(crate) client_state: State,
}

#[methods]
impl IncomingSession {
    /// Extract a session cookie from the incoming request, if it exists.
    ///
    /// If the cookie is not found, or if the cookie is invalid, this method will return `None`.
    #[request_scoped]
    pub fn extract(cookies: &RequestCookies<'_>, config: &SessionCookieConfig) -> Option<Self> {
        let cookie = cookies.get(&config.name)?;
        match serde_json::from_str::<WireClientState>(cookie.value()) {
            Ok(s) => Some(Self {
                id: s.session_id,
                client_state: s.user_values.into_owned(),
            }),
            Err(e) => {
                tracing::event!(
                    tracing::Level::WARN,
                    { ERROR_MESSAGE } = error_message(&e),
                    { ERROR_DETAILS } = error_details(&e),
                    "Invalid client state for session, creating a new session."
                );
                None
            }
        }
    }

    /// Build an [`IncomingSession`] instance from its parts.
    pub fn from_parts(id: SessionId, state: State) -> Self {
        Self {
            id,
            client_state: state,
        }
    }
}
