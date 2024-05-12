use serde::Serialize;

// The global state
#[derive(Clone, Serialize)]
pub enum Event {
    Attack((i64, i64), (i64, i64), u64), // bullet-type, src, target
}
