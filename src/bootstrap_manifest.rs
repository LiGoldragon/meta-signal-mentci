//! Producer-owned authority state for the Mentci meta-policy Interface.
//!
//! Every identity and canonical-order value is an allocated opaque seat.
//! None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}
impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}
impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}
pub const AUTHORITY_IDENTITY: [u8; 32] = [
    126, 218, 79, 147, 219, 186, 155, 144, 24, 204, 180, 153, 117, 49, 73, 159, 48, 217, 176, 26,
    18, 34, 83, 66, 212, 122, 168, 95, 60, 39, 225, 233,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 64537;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 21208;
pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 58519, 0x9d2a9c169333882d);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 1589, 0xc8a591e28fcac727);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 21643, 0x20791c2cd2a80ab1);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 8516, 0x5e890d2378c4c54b);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 59525, 0x2d6e781ce90aad75);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 60270, 0x05f93d8c30055daf);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 36013, 0x54eeb837ddacf479);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 62083, 0xb26066db30f1b453);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 4776, 0x07bc58e274f0a3bd);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 20464, 0x621df5ddc5f92d37);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 1701, 0x350c67df90cbbf41);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 7150, 0xa594da1714b96c5b);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 44471, 0xb34fecaa918c8b05);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 44028, 0x5ae564e2a46255bf);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 16741, 0xea41c9913add8b09);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 53391, 0xf9919b05825a0d63);
pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    48254, 57609, 34311, 26912, 52530, 57983, 52453, 36213, 49245, 45462,
];
pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "MentciMetaRequest", 40250, 0x86183362f5b931cf),
    DeclarationSeat::new(None, "MentciMetaReply", 31269, 0xa09bfbd3f292a599),
    DeclarationSeat::new(None, "PersonaName", 10594, 0x5f6d12487f928a73),
    DeclarationSeat::new(None, "PersonaKeyLabel", 53191, 0xfe454a6f71e726dd),
    DeclarationSeat::new(None, "ConfigurationGeneration", 58353, 0xe5765498f7d32557),
    DeclarationSeat::new(None, "ComponentSocketKind", 54958, 0x2963d7e9f6bc3461),
    DeclarationSeat::new(None, "ComponentSocket", 64749, 0x7eaada77e89aa67b),
    DeclarationSeat::new(None, "PersonaIdentity", 64708, 0x1e6bb84435b21225),
    DeclarationSeat::new(None, "NotificationClient", 19006, 0x4119e06401baf1df),
    DeclarationSeat::new(None, "MentciDaemonConfiguration", 65036, 0x04c2e7bf67e54429),
    DeclarationSeat::new(None, "Configured", 53089, 0x2c219cc43f5c2b83),
    DeclarationSeat::new(
        None,
        "ConfigurationRejectionReason",
        2928,
        0x81364d0e4a328e6d,
    ),
    DeclarationSeat::new(None, "ConfigurationRejected", 22627, 0x9acf35fd32e0b767),
    DeclarationSeat::new(None, "OperationKind", 46806, 0xb411b05288baf4f1),
    DeclarationSeat::new(None, "UnimplementedReason", 56333, 0x181a637c1209398b),
    DeclarationSeat::new(None, "RequestUnimplemented", 30841, 0xa43e6084daa7bbb5),
    DeclarationSeat::new(Some(40250), "Configure", 39259, 0x52dc9ab7705895ef),
    DeclarationSeat::new(
        Some(31269),
        "ConfigurationApplied",
        57718,
        0x2a0d9942522e66b9,
    ),
    DeclarationSeat::new(
        Some(31269),
        "ConfigurationRefused",
        9266,
        0x445b2c5bc6b7f093,
    ),
    DeclarationSeat::new(
        Some(31269),
        "OperationUnimplemented",
        53156,
        0x06d39099a4d4b9fd,
    ),
    DeclarationSeat::new(Some(54958), "Mentci", 54609, 0x12b62ff42d5aad77),
    DeclarationSeat::new(Some(54958), "MetaMentci", 13993, 0x3f5eb3976ff4b981),
    DeclarationSeat::new(Some(54958), "Criome", 20872, 0x1580339debe2709b),
    DeclarationSeat::new(Some(54958), "MetaCriome", 21065, 0x3b689138bd80a945),
    DeclarationSeat::new(Some(54958), "Introspect", 7207, 0x564ee3f7c3c31dff),
    DeclarationSeat::new(Some(54958), "MetaIntrospect", 22016, 0xbbf85a574d070d49),
    DeclarationSeat::new(Some(19006), "StatusBar", 26809, 0x56a1f4c418e6d9a3),
    DeclarationSeat::new(Some(19006), "Popup", 60882, 0xb2478a5364f6a98d),
    DeclarationSeat::new(Some(19006), "Email", 40945, 0xd9b08edf5c920787),
    DeclarationSeat::new(
        Some(2928),
        "ManagerAuthorityRequired",
        9313,
        0x26631bf330228211,
    ),
    DeclarationSeat::new(
        Some(2928),
        "MalformedConfiguration",
        19427,
        0xb388227a5e874bab,
    ),
    DeclarationSeat::new(Some(2928), "StoreUnavailable", 33344, 0xeff6c067eb85dad5),
    DeclarationSeat::new(Some(46806), "Configure", 24439, 0x654a4f3a686b8a0f),
    DeclarationSeat::new(Some(56333), "NotBuiltYet", 31367, 0x3fd587e5684837d9),
    DeclarationSeat::new(Some(56333), "DependencyNotReady", 3354, 0x0f285d374769e6b3),
];
