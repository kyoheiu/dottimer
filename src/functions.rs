use super::errors::MyError;
use super::state::*;
use regex::Regex;

pub fn input_to_number(input: String) -> Result<u16, MyError> {
    if input.is_empty() {
        return Err(MyError::ParseInputError {
            msg: "Please enter something here.".to_string(),
        });
    }
    let result = input.trim().parse()?;
    Ok(result)
}

pub fn input_to_numvec(s: String, max: u16) -> Result<Vec<u16>, MyError> {
    if s.is_empty() {
        return Err(MyError::ParseInputError {
            msg: "Please enter something here.".to_string(),
        });
    }
    let mut result: Vec<u16> = vec![];
    for n in s.split_ascii_whitespace() {
        let x = n.parse()?;
        if x >= 1 && x <= max {
            result.push(x);
        } else {
            return Err(MyError::ParseInputError {
                msg: "Input must be in the appropriate range.".to_string(),
            });
        }
    }
    Ok(result)
}

pub fn num_to_kinds(x: u16) -> Result<Vec<Kind>, MyError> {
    let mut result = vec![];
    match x {
        1 => result.push(Kind::Monotonic),
        2 => result.push(Kind::Realtime),
        3 => {
            result.push(Kind::Monotonic);
            result.push(Kind::Realtime);
        }
        _ => {
            return Err(MyError::ParseInputError {
                msg: "Input must be in 1 to 3.".to_string(),
            })
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

    for kind in state.timer_kind {
        match kind {
            Kind::Monotonic => {
                if let Some(ref vec) = state.monotonic_kind {
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
                        result.push_str(span);
                        result.push('\n');
                    }
                }
            }
            Kind::Realtime => {
                for calendar in state.calendar.as_ref().unwrap() {
                    result.push_str("OnCalendar=");
                    result.push_str(calendar);
                    result.push('\n');
                }
            }
        }
    }
    if state.accuracy.is_some() {
        result.push_str("AccuracySec=");
        result.push_str(&state.accuracy.unwrap());
        result.push('\n');
    }
    if state.randomized_delay.is_some() {
        result.push_str("RandomizedDelaySec=");
        result.push_str(&state.randomized_delay.unwrap());
        result.push('\n');
    }
    if state.fixed_random_delay {
        result.push_str("FixedRandomDelay=true");
        result.push('\n');
    }
    if state.on_clock_change {
        result.push_str("OnClockChange=true");
        result.push('\n');
    }
    if state.on_timezone_change {
        result.push_str("OnTimezoneChange=true");
        result.push('\n');
    }
    if state.persistent {
        result.push_str("Persistent=true");
        result.push('\n');
    }
    if state.wake_system {
        result.push_str("WakeSystem=true");
        result.push('\n');
    }
    if state.remain_after_elapse {
        result.push_str("RemainAfterElapse=true");
        result.push('\n');
    }

    result.push_str("\n[Install]\nWantedBy=timers.target");

    result
}

pub fn to_dow(input: String) -> Vec<String> {
    let mut result = vec![];
    for words in input.split_whitespace() {
        match words.to_lowercase().as_str() {
            "mon" => result.push("Mon".to_string()),
            "tue" => result.push("Tue".to_string()),
            "wed" => result.push("Wed".to_string()),
            "thu" => result.push("Thu".to_string()),
            "fri" => result.push("Fri".to_string()),
            "sat" => result.push("Sat".to_string()),
            "sun" => result.push("Sun".to_string()),
            _ => continue,
        }
    }
    result
}

pub fn to_year(input: String) -> Result<Vec<String>, MyError> {
    let mut result = vec![];
    let re = Regex::new(r"^[0-9\.\s]*$")?;
    let multi_year_re = Regex::new(r"^[0-9]+\.\.[0-9]+$")?;
    if !re.is_match(&input) {
        return Err(MyError::ParseInputError {
            msg: "cannot parse input for year".to_string(),
        });
    }
    for word in input.split_whitespace() {
        if word.contains('.') {
            if multi_year_re.is_match(word) {
                result.push(word.to_string());
            } else {
                return Err(MyError::ParseInputError {
                    msg: "cannot parse input for year".to_string(),
                });
            }
        } else {
            result.push(word.to_string());
        }
    }
    Ok(result)
}

pub fn to_monthday(input: String) -> Result<Vec<String>, MyError> {
    let mut result = vec![];
    let re = Regex::new(r"^[0-9\.\s]*$")?;
    let single_md_re = Regex::new(r"^[0-9]{1,2}$")?;
    let multi_md_re = Regex::new(r"^[0-9]{1,2}\.\.[0-9]{1,2}$")?;
    if !re.is_match(&input) {
        return Err(MyError::ParseInputError {
            msg: "cannot parse input for month or day".to_string(),
        });
    }
    for word in input.split_whitespace() {
        if word.contains('.') {
            if multi_md_re.is_match(word) {
                result.push(word.to_string());
            } else {
                return Err(MyError::ParseInputError {
                    msg: "cannot parse input for month or day".to_string(),
                });
            }
        } else if single_md_re.is_match(word) {
            result.push(word.to_string());
        } else {
            return Err(MyError::ParseInputError {
                msg: "cannot parse input for month or day".to_string(),
            });
        }
    }
    Ok(result)
}

pub fn to_time(input: String) -> Result<String, MyError> {
    if input.trim().is_empty() {
        return Ok("00:00:00".to_string());
    }
    let re = Regex::new(r"^[0-9]{1,2}:[0-9]{1,2}:[0-9]{1,2}$")?;
    if !re.is_match(&input) {
        return Err(MyError::ParseInputError {
            msg: "cannot parse input for time".to_string(),
        });
    }
    Ok(input)
}

pub fn format_to_calendar(format: Format) -> String {
    let mut result = String::new();
    if !(format.dow.is_empty()) {
        for x in format.dow {
            result.push_str(&x);
            result.push(',')
        }
        result.pop();
        result.push(' ');
    }

    if !(format.year.is_empty()) {
        for x in format.year {
            result.push_str(&x);
            result.push(',');
        }
        result.pop();
        result.push('-');
    } else {
        result.push_str("*-");
    }

    if !(format.month.is_empty()) {
        for x in format.month {
            result.push_str(&x);
            result.push(',');
        }
        result.pop();
        result.push('-');
    } else {
        result.push_str("*-");
    }

    if !(format.day.is_empty()) {
        for x in format.day {
            result.push_str(&x);
            result.push(',');
        }
        result.pop();
        result.push(' ');
    } else {
        result.push_str("* ");
    }

    if format.time.is_empty() {
        result.push_str("00:00:00");
    } else {
        result.push_str(&format.time);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_to_numvec() {
        assert_eq!(vec![1, 3], input_to_numvec("1 3".to_string(), 3).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_input_to_numvec2_fails() {
        let _result = input_to_numvec("13".to_string(), 5).unwrap();
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
            timer_kind: vec![Kind::Monotonic],
            monotonic_kind: Some(vec![
                (MonotonicKind::OnActive, "4min".to_string()),
                (MonotonicKind::OnBoot, "5min".to_string()),
            ]),
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
    fn test_to_monotonic_timer_with_option() {
        let input = State {
            desciprtion: "Example Timer".to_string(),
            timer_kind: vec![Kind::Monotonic],
            monotonic_kind: Some(vec![
                (MonotonicKind::OnActive, "4min".to_string()),
                (MonotonicKind::OnBoot, "5min".to_string()),
            ]),
            calendar: None,
            format: None,
            accuracy: Some("1us".to_string()),
            randomized_delay: Some("1us".to_string()),
            fixed_random_delay: true,
            on_clock_change: true,
            on_timezone_change: true,
            persistent: true,
            wake_system: true,
            remain_after_elapse: true,
        };
        assert_eq!(
            "[Unit]
Description=Example Timer

[Timer]
OnActiveSec=4min
OnBootSec=5min
AccuracySec=1us
RandomizedDelaySec=1us
FixedRandomDelay=true
OnClockChange=true
OnTimezoneChange=true
Persistent=true
WakeSystem=true
RemainAfterElapse=true

[Install]
WantedBy=timers.target",
            to_timer(input)
        );
    }

    #[test]
    fn test_to_realtime_timer() {
        let input = State {
            desciprtion: "Example Timer2".to_string(),
            timer_kind: vec![Kind::Realtime],
            monotonic_kind: None,
            calendar: Some(vec!["Mon *-*-* 00:00:00".to_string()]),
            format: None,
            accuracy: None,
            randomized_delay: None,
            fixed_random_delay: false,
            on_clock_change: false,
            on_timezone_change: false,
            persistent: false,
            wake_system: false,
            remain_after_elapse: false,
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

    #[test]
    fn test_to_dow() {
        let input = "Mon tue FrI Sum";
        assert_eq!(
            vec!["Mon".to_string(), "Tue".to_string(), "Fri".to_string()],
            to_dow(input.to_string())
        );
        let input2 = "";
        assert_eq!(Vec::<String>::new(), to_dow(input2.to_string()));
    }

    #[test]
    fn test_to_year() {
        let input = "2022 2024 2026..2030";
        assert_eq!(
            vec![
                "2022".to_string(),
                "2024".to_string(),
                "2026..2030".to_string()
            ],
            to_year(input.to_string()).unwrap()
        );
    }
    #[test]
    #[should_panic]
    fn test_to_year_fails() {
        let input2 = "abcd 202*";
        let _result = to_year(input2.to_string()).unwrap();
    }

    #[test]
    fn test_to_month() {
        let input = "1 3 5..11";
        assert_eq!(
            vec!["1".to_string(), "3".to_string(), "5..11".to_string()],
            to_monthday(input.to_string()).unwrap()
        );
    }
    #[test]
    #[should_panic]
    fn test_to_month_fails() {
        let input2 = "3 1..1000";
        let _result = to_monthday(input2.to_string()).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_to_month_fails2() {
        let input2 = "2424";
        let _result = to_monthday(input2.to_string()).unwrap();
    }

    #[test]
    fn test_to_time() {
        let input = "1:01:25";
        assert_eq!("1:01:25".to_string(), to_time(input.to_string()).unwrap());
    }
    #[test]
    #[should_panic]
    fn test_to_time_fails() {
        let input2 = "12:3A:00";
        let _result = to_time(input2.to_string()).unwrap();
    }

    #[test]
    fn test_format_to_calendar() {
        let input = Format {
            dow: vec!["Mon".to_string(), "Wed".to_string()],
            year: vec!["2022..2024".to_string()],
            month: vec!["1".to_string(), "3..5".to_string()],
            day: vec!["24".to_string()],
            time: "00:00:00".to_string(),
        };
        assert_eq!(
            "Mon,Wed 2022..2024-1,3..5-24 00:00:00".to_string(),
            format_to_calendar(input)
        );
    }

    #[test]
    fn test_format_to_calendar2() {
        let input = Format {
            dow: vec![],
            year: vec![],
            month: vec![],
            day: vec![],
            time: "12:00:00".to_string(),
        };
        assert_eq!("*-*-* 12:00:00".to_string(), format_to_calendar(input));
    }

    #[test]
    fn test_vec_to_kinds() {
        let input = 3;
        assert_eq!(
            vec![Kind::Monotonic, Kind::Realtime],
            num_to_kinds(input).unwrap()
        );
    }

    #[test]
    fn test_input_to_number() {
        let input = "3".to_string();
        assert_eq!(3, input_to_number(input).unwrap());
    }
}
