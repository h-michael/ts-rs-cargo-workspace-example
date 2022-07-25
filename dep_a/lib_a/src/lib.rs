use ts_rs::TS;

#[derive(TS)]
#[ts(
    export,
    export_to = "../../bindings/"
)]
pub struct A {}

#[derive(TS)]
#[ts(
    export,
    export_to = "../../bindings/"
)]
pub struct ATest {
    pub a: A,
}
