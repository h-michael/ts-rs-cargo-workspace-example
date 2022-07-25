use ts_rs::TS;

#[derive(TS)]
#[ts(
    export,
    export_to = "../../bindings/"
)]
pub struct B {}
