# INTENT - meta-signal-mentci

`meta-signal-mentci` is the meta policy contract for the Mentci daemon.

Ordinary programmable-UI traffic lives in `signal-mentci`. This crate owns the
typed `Configure` request and its replies: the binary startup/reconfiguration
message that tells a Mentci daemon where to listen, which home criome socket to
use for verdict signing, which persona identity it represents, and which
notification clients are enabled.

Daemons accept binary rkyv startup/meta messages, not inline NOTA or `.nota`
paths. This crate defines that binary message type. The daemon, state, key
custody, rendering clients, and criome verdict signing live elsewhere.
