# meta-signal-mentci - architecture

`meta-signal-mentci` is the owner/meta configuration contract for the Mentci
daemon. Its producer-owned `ethos/interface.ethos` is the sole structural
authority; strict projection supplies encoded Rust identities, Dotos text,
rkyv storage, and the bound `signal-frame` envelope.

## 0.5 · Direction

`meta-signal-mentci` is the meta policy contract for the Mentci daemon. Ordinary programmable-UI traffic lives in `signal-mentci`; this crate carries the single `Configure` request that provides the binary startup/reconfiguration message — socket endpoints, home criome socket, persona identity, and enabled notification clients.

## Owned

- `Configure MentciDaemonConfiguration`.
- `Configured`, `ConfigurationRejected`, and `RequestUnimplemented` replies.
- `MentciDaemonConfiguration`, `PersonaIdentity`, and `NotificationClient`.
- The placement of producer-owned `signal-standard` `StandardSocket` and
  `ComponentKind` values in Mentci configuration.

## Not Owned

- Working UI traffic. That lives in `signal-mentci`.
- The daemon runtime, actors, durable state, and sockets.
- Criome key custody and verdict signing.

## Invariants

- There is one meta verb: `Configure`.
- Configuration generation is a plain monotonic counter.
- The contract is wire-only: no daemon clients, runtime policy, redb tables, or
  actors.
