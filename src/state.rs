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
    pub accuracy: Option<String>,
    pub randomized_delay: Option<String>,
    pub fixed_random_delay: bool,
    pub on_clock_change: bool,
    pub on_timezone_change: bool,
    pub persistent: bool,
    pub wake_system: bool,
    pub remain_after_elapse: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            desciprtion: String::new(),
            timer_kind: vec![Kind::Monotonic],
            monotonic_kind: None,
            calendar: None,
            format: None,
            accuracy: None,
            randomized_delay: None,
            fixed_random_delay: false,
            on_clock_change: false,
            on_timezone_change: false,
            persistent: false,
            wake_system: false,
            remain_after_elapse: false,
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
