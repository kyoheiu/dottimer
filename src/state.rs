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
    pub desciprtion: String,
    pub timer_kind: Kind,
    pub monotonic_kind: Option<Vec<(MonotonicKind, String)>>,
    pub calendar: Option<String>,
    pub format: Option<Format>,
}

impl State {
    pub fn new() -> Self {
        State {
            desciprtion: String::new(),
            timer_kind: Kind::Monotonic,
            monotonic_kind: None,
            calendar: None,
            format: None,
        }
    }
}

#[derive(Debug)]
pub struct Format {
    pub dow: Vec<DoW>,
    pub year: Vec<usize>,
    pub month: Vec<usize>,
    pub day: Vec<usize>,
    pub time: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DoW {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}
