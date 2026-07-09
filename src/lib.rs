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

impl ComponentSocket {
    pub fn new(
        component_socket_kind: ComponentSocketKind,
        standard_socket: StandardSocket,
    ) -> Self {
        Self {
            component_socket_kind,
            standard_socket,
        }
    }
}

impl PersonaIdentity {
    pub fn new(
        persona_name: PersonaName,
        component_kind: ComponentKind,
        persona_key_label: PersonaKeyLabel,
    ) -> Self {
        Self {
            persona_name,
            component_kind,
            persona_key_label,
        }
    }
}

impl MentciDaemonConfiguration {
    pub fn new(
        component_sockets: Vec<ComponentSocket>,
        persona_identity: PersonaIdentity,
        notification_clients: Vec<NotificationClient>,
    ) -> Self {
        Self {
            component_sockets: ComponentSockets::new(component_sockets),
            persona_identity,
            notification_clients: NotificationClients::new(notification_clients),
        }
    }

    pub fn component_sockets(&self) -> &[ComponentSocket] {
        self.component_sockets.payload().as_slice()
    }

    pub fn component_socket(
        &self,
        component_socket_kind: ComponentSocketKind,
    ) -> Option<&ComponentSocket> {
        self.component_sockets().iter().find(|component_socket| {
            component_socket.component_socket_kind == component_socket_kind
        })
    }
}
