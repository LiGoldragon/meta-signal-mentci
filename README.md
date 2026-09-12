# meta-signal-mentci

The meta Signal contract for Mentci: configuration only.

`Configure` carries the whole daemon configuration — which socket Mentci
seats and under which component-socket kind, which persona identity it speaks
as, and where it sends notifications. The reply is the applied generation, a
typed refusal, or a typed "not built yet".

`ethos/signal.ethos` is the schema authority. `build.rs` regenerates the
projection with `ethos-zero` and asserts it against the committed
`src/generated/signal.rs`, so the two can never drift.

`examples/canonical.datom` holds one canonical Datom value per line, written
by the codec and never spelled by hand; `tests/contract.rs` asserts every line
is exactly what the codec writes and that each actualizes back into exactly
one of `Query` and `Response`.

Everything Mentci presents to a human — questions, verdicts, panes,
notifications, the parked-request views — lives in the ordinary contract,
`signal-mentci`.
