use schema_rust::build::ContractCrateBuild;

fn main() {
    ContractCrateBuild::from_environment(
        "meta-signal-mentci",
        "0.1.0",
        "META_SIGNAL_MENTCI_UPDATE_SCHEMA_ARTIFACTS",
    )
    .expect_fresh();
}
