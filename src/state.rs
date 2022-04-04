#[derive(Debug)]
pub enum Kind {
    Monotonic,
    Realtime,
}

#[derive(Debug)]
pub enum MonotonicKind {
    OnActive,
    OnBoot,
    OnStartup,
    OnUnitActive,
    OnUnitInactive,
}

#[derive(Debug)]
pub struct State {
    pub timer_kind: Kind,
    pub monotonic_kind: Option<Vec<(MonotonicKind, String)>>,
}

impl State {
    pub fn new() -> Self {
        State {
            timer_kind: Kind::Monotonic,
            monotonic_kind: None,
        }
    }
}
