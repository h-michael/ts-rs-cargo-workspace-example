use ts_rs::TS;
use lib_a;

#[derive(TS)]
#[ts(
    export,
    export_to = "../bindings/"
)]
pub struct ABC {
    pub a: lib_a::A,
    pub b: lib_b::B,
    pub c: C,
}

#[derive(TS)]
#[ts(
    export,
    export_to = "../bindings/"
)]
pub struct C {}
