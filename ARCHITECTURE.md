# meta-signal-mentci architecture

## Center

This repository owns the privileged Mentci surface: what a manager may set,
and what it is told in return. It owns no policy, no storage, no daemon, and
nothing a human ever sees.

## Authority and projection

`ethos/signal.ethos` is the sole textual source — a `Signal` root holding the
import list, the request variants, the reply variants, and the type
declarations. `ethos-zero` projects it into `src/generated/signal.rs`, which
is committed; `build.rs` generates afresh and asserts equality, so a build
cannot succeed while source and projection differ.

`src/lib.rs` re-exports the projection and nothing else. Every name a
consumer writes is a name the ethos declares.

The request and reply roots are named `Query` and `Response` — `ethos-zero`
names them, not this contract.

## Two socket vocabularies, and why both

`ComponentSocketKind` says which of Mentci's own sockets a configuration
seats: the ordinary and meta sockets of Mentci, Criome and Introspect.
`signal::ComponentKind` says which component in the estate a persona identity
belongs to. They are different questions, so they are different types; the
first is Mentci's own, the second is the estate's and is imported.

`signal::ComponentKind` carries no `Mentci` variant. That is the shared
taxonomy's gap, not this contract's, and this contract does not paper over it
with a local copy.

## Boundaries

`signal` for the frame, the wire framing and the taxonomy; `rkyv` for the
archive; under the optional `datom` feature, `datom-codec` and `protos` for
the Datom text projection. Nothing else, and nothing here reaches a
filesystem, a socket, or a clock.
