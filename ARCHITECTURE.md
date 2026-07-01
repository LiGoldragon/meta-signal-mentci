# meta-signal-mentci - architecture

`meta-signal-mentci` is the owner/meta configuration contract for the Mentci
daemon. It is schema-derived wire vocabulary over `signal-frame`, with optional
NOTA projection for edge clients.

## 0.5 · Direction

`meta-signal-mentci` is the meta policy contract for the Mentci daemon. Ordinary programmable-UI traffic lives in `signal-mentci`; this crate carries the single `Configure` request that provides the binary startup/reconfiguration message — socket endpoints, home criome socket, persona identity, and enabled notification clients. Daemons accept binary rkyv startup/meta messages, not inline NOTA or `.nota` paths.

## Owned

- `Configure MentciDaemonConfiguration`.
- `Configured`, `ConfigurationRejected`, and `RequestUnimplemented` replies.
- `MentciDaemonConfiguration`, `PersonaIdentity`, `NotificationClient`, and the
  local `StandardSocket`/`ComponentKind` stand-ins until `signal-standard` is a
  remote dependency.

## Not Owned

- Working UI traffic. That lives in `signal-mentci`.
- The daemon runtime, actors, durable state, and sockets.
- Criome key custody and verdict signing.

## Invariants

- There is one meta verb: `Configure`.
- Configuration generation is a plain monotonic counter.
- The contract is wire-only: no daemon clients, runtime policy, redb tables, or
  actors.
