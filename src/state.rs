#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    pub desciprtion: String,
    pub timer_kind: Vec<Kind>,
    pub monotonic_kind: Option<Vec<(MonotonicKind, String)>>,
    pub calendar: Option<Vec<String>>,
    pub format: Option<Vec<Format>>,
}

impl State {
    pub fn new() -> Self {
        State {
            desciprtion: String::new(),
            timer_kind: vec![Kind::Monotonic],
            monotonic_kind: None,
            calendar: None,
            format: None,
        }
    }
}

#[derive(Debug)]
pub struct Format {
    pub dow: Vec<String>,
    pub year: Vec<String>,
    pub month: Vec<String>,
    pub day: Vec<String>,
    pub time: String,
}
