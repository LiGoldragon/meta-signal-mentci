//! The contract's own witness: every canonical value survives the wire, and
//! every canonical Datom line is the codec's own text for one of them.

use meta_signal_mentci::{
    ByteViewable, ComponentKind, ComponentSocket, ComponentSocketKind, ConfigurationRejected,
    ConfigurationRejectionReason, Configured, MentciDaemonConfiguration, NotificationClient,
    OperationKind, PersonaIdentity, Query, RequestUnimplemented, Response, Restorable, Signal,
    Signalizable, StandardSocket, UnimplementedReason,
};

fn configuration() -> MentciDaemonConfiguration {
    MentciDaemonConfiguration {
        component_socket: ComponentSocket {
            component_socket_kind: ComponentSocketKind::Mentci,
            standard_socket: StandardSocket::UnixSocket(String::from("/run/mentci/data.sock")),
        },
        persona_identity: PersonaIdentity {
            persona_name: String::from("operator"),
            component_kind: ComponentKind::Persona,
            persona_key_label: String::from("operator-key"),
        },
        notification_client: NotificationClient::StatusBar,
    }
}

fn canonical_queries() -> Vec<Query> {
    vec![Query::Configure(configuration())]
}

fn canonical_responses() -> Vec<Response> {
    vec![
        Response::ConfigurationApplied(Configured {
            configuration_generation: 7,
        }),
        Response::ConfigurationRefused(ConfigurationRejected {
            configuration_rejection_reason: ConfigurationRejectionReason::ManagerAuthorityRequired,
        }),
        Response::OperationUnimplemented(RequestUnimplemented {
            operation_kind: OperationKind::Configure,
            unimplemented_reason: UnimplementedReason::DependencyNotReady,
        }),
    ]
}

#[test]
fn every_canonical_query_restores_from_fresh_peer_bytes() {
    for query in canonical_queries() {
        let received = Signal::<Query>::from(query.signalize().expect("archive").bytes().to_vec());
        assert_eq!(received.restore().expect("restore"), query);
    }
}

#[test]
fn every_canonical_response_restores_from_fresh_peer_bytes() {
    for response in canonical_responses() {
        let received =
            Signal::<Response>::from(response.signalize().expect("archive").bytes().to_vec());
        assert_eq!(received.restore().expect("restore"), response);
    }
}

#[test]
fn a_malformed_archive_is_refused() {
    assert!(Signal::<Query>::from(vec![1, 2, 3]).restore().is_err());
    assert!(Signal::<Response>::from(vec![1, 2, 3]).restore().is_err());
}

#[cfg(feature = "datom")]
mod canonical {
    use super::{canonical_queries, canonical_responses};
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use meta_signal_mentci::{Query, Response};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    const CANONICAL: &str = include_str!("../examples/canonical.datom");

    fn budget() -> Budget {
        Budget {
            remaining: 1 << 20,
            reader: ReaderBudget { remaining: 1 << 20 },
            depth: 0,
            maximum_depth: 256,
        }
    }

    fn lines() -> Vec<String> {
        CANONICAL
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with(';'))
            .map(str::to_string)
            .collect()
    }

    /// Rewrite `examples/canonical.datom` from the canonical values. The file
    /// is the codec's product, never text spelled by hand; run this whenever
    /// the canonical values change:
    ///
    /// ```text
    /// cargo test --features datom -- --ignored rewrite_the_canonical_file
    /// ```
    #[test]
    #[ignore = "writes the source tree; run deliberately when the values change"]
    fn rewrite_the_canonical_file() {
        let mut out = String::from("; Canonical Datom examples for meta-signal-mentci.\n");
        out.push_str("; Written by `rewrite_the_canonical_file`; never spelled by hand.\n\n");
        for query in canonical_queries() {
            out.push_str(&query.datomize(vec![]).protosize().textualize());
            out.push('\n');
        }
        for response in canonical_responses() {
            out.push_str(&response.datomize(vec![]).protosize().textualize());
            out.push('\n');
        }
        std::fs::write(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/canonical.datom"),
            out,
        )
        .expect("write canonical");
    }

    /// The canonical file is the codec's own text for the canonical values —
    /// never text spelled by hand.
    #[test]
    fn the_canonical_file_is_what_the_codec_writes() {
        let mut written: Vec<String> = Vec::new();
        for query in canonical_queries() {
            written.push(query.datomize(vec![]).protosize().textualize());
        }
        for response in canonical_responses() {
            written.push(response.datomize(vec![]).protosize().textualize());
        }
        assert_eq!(lines(), written);
    }

    /// Every line the file carries actualizes — as a request or as a reply,
    /// never as neither and never as both.
    #[test]
    fn every_canonical_line_actualizes_into_exactly_one_root() {
        let lines = lines();
        assert!(!lines.is_empty());
        for line in lines {
            let query = Potential::<Query>::from(line.clone())
                .actualize(&mut budget())
                .ok();
            let response = Potential::<Response>::from(line.clone())
                .actualize(&mut budget())
                .ok();
            match (query, response) {
                (Some(query), None) => {
                    assert_eq!(query.datomize(vec![]).protosize().textualize(), line)
                }
                (None, Some(response)) => {
                    assert_eq!(response.datomize(vec![]).protosize().textualize(), line)
                }
                (Some(_), Some(_)) => panic!("ambiguous canonical line: {line}"),
                (None, None) => panic!("unreadable canonical line: {line}"),
            }
        }
    }
}
