use super::errors::MyError;
use super::state::*;

pub fn input_to_numvec(s: String) -> Result<Vec<u16>, MyError> {
    let mut result: Vec<u16> = vec![];
    for n in s.split_ascii_whitespace() {
        let x = n.parse()?;
        if x >= 1 && x <= 5 {
            result.push(x);
        }
    }
    Ok(result)
}

pub fn to_human(input: String) -> String {
    let mut result = String::new();
    for line in input.lines() {
        let line = line.trim_start();
        if let Some(char) = line.chars().next() {
            if char == 'H' {
                if let Some(pair) = line.split_once(' ') {
                    result = pair.1.to_string();
                }
            }
        }
    }
    result
}

pub fn to_normalized(input: String) -> String {
    let mut result = String::new();
    for line in input.lines() {
        let c = line.chars().next();
        if c == Some('N') {
            result = line.chars().skip(17).collect();
        }
    }
    result
}

pub fn to_timer(state: State) -> String {
    let mut result = "[Unit]
Description="
        .to_string();

    result.push_str(&state.desciprtion);
    result.push_str("\n\n[Timer]\n");

    match state.timer_kind {
        Kind::Monotonic => {
            if let Some(vec) = state.monotonic_kind {
                for (kind, span) in vec {
                    match kind {
                        MonotonicKind::OnActive => {
                            result.push_str("OnActiveSec=");
                        }
                        MonotonicKind::OnBoot => {
                            result.push_str("OnBootSec=");
                        }
                        MonotonicKind::OnStartup => {
                            result.push_str("OnStartupSec=");
                        }
                        MonotonicKind::OnUnitActive => {
                            result.push_str("OnUnitActiveSec=");
                        }
                        MonotonicKind::OnUnitInactive => {
                            result.push_str("OnUnitInactiveSec=");
                        }
                    }
                    result.push_str(&span);
                    result.push('\n');
                }
            }
        }
        Kind::Realtime => {
            result.push_str("OnCalendar=");
            result.push_str(&state.calendar.unwrap());
            result.push('\n');
        }
    }

    result.push_str("\n[Install]\nWantedBy=timers.target");

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_to_numvec() {
        assert_eq!(vec![1, 3], input_to_numvec("1 3".to_string()).unwrap());
        assert_ne!(vec![1, 3], input_to_numvec("13".to_string()).unwrap());
    }

    #[test]
    fn test_to_human() {
        let input = "Original: 240
        μs: 240000000
     Human: 4min
        ";
        assert_eq!("4min", to_human(input.to_string()));
    }

    #[test]
    fn test_to_normalized() {
        let input = "  Original form: Weekly
Normalized form: Mon *-*-* 00:00:00
    Next elapse: Mon 2022-04-11 00:00:00 JST
       (in UTC): Sun 2022-04-10 15:00:00 UTC
       From now: 5 days left
        ";
        assert_eq!("Mon *-*-* 00:00:00", to_normalized(input.to_string()));
    }

    #[test]
    fn test_to_monotonic_timer() {
        let input = State {
            desciprtion: "Example Timer".to_string(),
            timer_kind: Kind::Monotonic,
            monotonic_kind: Some(vec![
                (MonotonicKind::OnActive, "4min".to_string()),
                (MonotonicKind::OnBoot, "5min".to_string()),
            ]),
            calendar: None,
        };
        assert_eq!(
            "[Unit]
Description=Example Timer

[Timer]
OnActiveSec=4min
OnBootSec=5min

[Install]
WantedBy=timers.target",
            to_timer(input)
        );
    }

    #[test]
    fn test_to_realtime_timer() {
        let input = State {
            desciprtion: "Example Timer2".to_string(),
            timer_kind: Kind::Realtime,
            monotonic_kind: None,
            calendar: Some("Mon *-*-* 00:00:00".to_string()),
        };
        assert_eq!(
            "[Unit]
Description=Example Timer2

[Timer]
OnCalendar=Mon *-*-* 00:00:00

[Install]
WantedBy=timers.target",
            to_timer(input)
        );
    }
}
