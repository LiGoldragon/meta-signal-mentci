use dotos::{DotosEncode, DotosSource};
use meta_signal_mentci::*;

fn exchange() -> signal_frame::ExchangeIdentifier {
    signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(1),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::first(),
    )
}

#[test]
fn authority_projected_request_round_trips_through_dotos_and_the_bound_frame() {
    let request = z2VXjF::z2VXSA(z2Vf6b {
        field_0: z2Vf1e {
            field_0: z2Vc6q::z2Vbzp,
            field_1: signal_standard::schema::lib::z2VduW::z2VUkE(
                signal_standard::schema::lib::z2VXNY::new("/run/fixture.sock".to_owned()),
            ),
        },
        field_1: z2Vezw {
            field_0: z2VNuw::new("fixture".to_owned()),
            field_1: signal_standard::schema::lib::z2VWWD::z2VSDw,
            field_2: z2VbaN::new("fixture".to_owned()),
        },
        field_2: z2VRQy::z2VTjW,
    });
    let text = request.to_dotos();
    assert_eq!(
        DotosSource::new(&text)
            .parse::<z2VXjF>()
            .expect("request Dotos decodes"),
        request
    );
    let encoded = request
        .clone()
        .encode_request_frame(exchange())
        .expect("request frame encodes");
    let (decoded_exchange, decoded) =
        ContractMarker::decode_single_request(&encoded).expect("request frame decodes");
    assert_eq!(decoded_exchange, exchange());
    assert_eq!(decoded, request);
}

#[test]
fn authority_projected_reply_round_trips_through_dotos_and_archive_storage() {
    let reply = z2VV4Q::z2VbZm(z2VUw2 {
        field_0: z2VZgH::z2VT2e,
        field_1: z2VcWY::z2VLm7,
    });
    let text = reply.to_dotos();
    assert_eq!(
        DotosSource::new(&text)
            .parse::<z2VV4Q>()
            .expect("reply Dotos decodes"),
        reply
    );
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).expect("reply archives");
    let recovered =
        rkyv::from_bytes::<z2VV4Q, rkyv::rancor::Error>(&archive).expect("reply recovers");
    assert_eq!(recovered, reply);
}
