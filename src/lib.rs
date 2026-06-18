//! Schema-derived meta signal contract for Mentci daemon configuration.
//!
//! Ordinary programmable-UI traffic lives in `signal-mentci`. This crate carries
//! the authenticated binary configuration surface for the Mentci daemon.

#[rustfmt::skip]
#[allow(clippy::large_enum_variant, dead_code, private_interfaces)]
pub mod schema;

pub use schema::lib::*;

impl ConfigurationGeneration {
    pub fn value(&self) -> u64 {
        *self.payload()
    }
}

impl SocketPath {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl StandardSocket {
    pub fn unix(path: impl Into<String>) -> Self {
        Self::new(SocketPath::new(path.into()))
    }
}

impl PersonaIdentity {
    pub fn new(
        persona: PersonaName,
        speaks_for: ComponentKind,
        signing_key: PersonaKeyLabel,
    ) -> Self {
        Self {
            persona,
            speaks_for,
            signing_key,
        }
    }
}

impl MentciDaemonConfiguration {
    pub fn new(
        socket_path: StandardSocket,
        home_criome_socket: StandardSocket,
        persona_identity: PersonaIdentity,
        notification_clients: Vec<NotificationClient>,
    ) -> Self {
        Self {
            socket_path,
            home_criome_socket,
            persona_identity,
            notification_clients: NotificationClients::new(notification_clients),
        }
    }
}
