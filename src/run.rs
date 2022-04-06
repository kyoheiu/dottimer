use crate::functions::*;

use super::errors::MyError;
use super::functions::input_to_numvec;
use super::messeages::*;
use super::state::*;
use std::io::stdout;
use std::io::Write;
use termion::color::*;
use termion::cursor::{self, DetectCursorPos};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::style;

pub fn run() -> Result<(), MyError> {
    let mut state = State::new();

    let mut calendar_vec = vec![];

    let mut stdout = stdout().into_raw_mode().unwrap();
    print!("{}", cursor::Hide);

    stdout.suspend_raw_mode()?;
    print!("{}", Fg(Blue));
    println!("{WELCOME}");
    println!("{ENTER_DESC}");
    print!("{}", Fg(Reset));

    print!("{}Description > {}{}", Fg(Yellow), cursor::Show, Fg(Reset));
    stdout.flush()?;

    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let mut keys = std::io::stdin().keys();

    stdin.read_line(&mut buffer)?;
    state.desciprtion = buffer.trim().to_string();

    print!("{}", Fg(Blue));
    println!("{TIMER_KIND}");
    print!("{}", Fg(Reset));

    print!("{}", Fg(Yellow));
    println!("{ENTER_KIND}");
    println!("{}", Fg(Reset));

    let mut kind = Kind::Monotonic;
    print!("{KIND_LIST}> ");
    stdout.flush()?;

    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    let mut kinds = input_to_number(buffer.trim().to_string());
    loop {
        if kinds.is_err() {
            print!("{}", Fg(Yellow));
            print!("{PARSE_ERROR}");
            print!("{}", Fg(Reset));
            stdout.flush()?;
            let mut buffer = String::new();
            stdin.read_line(&mut buffer)?;
            kinds = input_to_number(buffer.trim().to_string());
            continue;
        } else {
            break;
        }
    }

    state.timer_kind = num_to_kinds(kinds.unwrap())?;
    stdout.suspend_raw_mode()?;
    print!("{}", cursor::Hide);

    println!();

    for kind in &state.timer_kind {
        match *kind {
            Kind::Monotonic => {
                print!("{}{}", Fg(Blue), style::Bold);
                println!(":: Monotonic ::");
                print!("{}", style::Reset);
                print!("{}", Fg(Yellow));
                println!("{CHOOSE_MONOTONIC_KIND}");
                print!("{}", Fg(Blue));
                println!("{MONOTONIC_KIND}");
                print!("{}", Fg(Reset));

                print!("> {}", cursor::Show);
                stdout.flush()?;

                let mut buffer = String::new();
                stdin.read_line(&mut buffer)?;
                let mut chosen = input_to_numvec(buffer.trim().to_string(), 5);
                loop {
                    if chosen.is_err() {
                        print!("{}", Fg(Yellow));
                        print!("{PARSE_ERROR}");
                        print!("{}", Fg(Reset));
                        stdout.flush()?;
                        let mut buffer = String::new();
                        stdin.read_line(&mut buffer)?;
                        chosen = input_to_numvec(buffer.trim().to_string(), 5);
                        continue;
                    } else {
                        break;
                    }
                }
                let chosen = chosen?;

                // after this line, one loop should be used
                print!("{}", Fg(Blue));
                println!("{ENTER_SPAN}");
                print!("{}", Fg(Reset));

                let mut monotonic_vec: Vec<(MonotonicKind, String)> = vec![];
                let mut timespan = String::new();

                for i in chosen {
                    print!("{}", Fg(Yellow));
                    match i {
                        1 => print!("OnActiveSec > "),
                        2 => print!("OnBootSec > "),
                        3 => print!("OnStartupSec > "),
                        4 => print!("OnUnitActiveSec > "),
                        5 => print!("OnUnitInactiveSec > "),
                        _ => continue,
                    }
                    print!("{}", Fg(Reset));
                    stdout.flush()?;

                    loop {
                        let mut buffer = String::new();
                        stdin.read_line(&mut buffer)?;
                        let trimmed = buffer.trim();
                        let output = std::process::Command::new("systemd-analyze")
                            .args(["timespan", trimmed])
                            .output()?
                            .stdout;
                        let mut output = std::str::from_utf8(&output)?.to_string();
                        loop {
                            if output.trim().is_empty() {
                                print!("{}", Fg(Yellow));
                                print!("{PARSE_ERROR}");
                                print!("{}", Fg(Reset));
                                stdout.flush()?;
                                let mut buffer = String::new();
                                stdin.read_line(&mut buffer)?;
                                let trimmed = buffer.trim();
                                let re_output = std::process::Command::new("systemd-analyze")
                                    .args(["timespan", trimmed])
                                    .output()?
                                    .stdout;
                                output = std::str::from_utf8(&re_output)?.to_string();
                                continue;
                            } else {
                                break;
                            }
                        }

                        print!("{}", Fg(Green));
                        println!("-------------------");
                        print!("{output}");
                        println!("-------------------");
                        print!("{}", Fg(Yellow));
                        print!("{OK_YN}");
                        print!("{}", Fg(Reset));
                        stdout.flush()?;

                        stdout.activate_raw_mode()?;
                        let input = keys.next();
                        if let Some(Ok(key)) = input {
                            match key {
                                Key::Char('\n') | Key::Char('y') | Key::Char('Y') => {
                                    stdout.suspend_raw_mode()?;
                                    print!("{}", cursor::Left(100));
                                    println!();
                                    timespan = output.clone();
                                    break;
                                }
                                _ => {
                                    stdout.suspend_raw_mode()?;
                                    print!("{}", cursor::Left(100));
                                    print!("{}", Fg(Yellow));
                                    println!();
                                    print!("{ENTER_AGAIN}");
                                    print!("{}", Fg(Reset));
                                    stdout.flush()?;
                                    continue;
                                }
                            }
                        }
                    }

                    let human = to_human(timespan);
                    match i {
                        1 => monotonic_vec.push((MonotonicKind::OnActive, human)),
                        2 => monotonic_vec.push((MonotonicKind::OnBoot, human)),
                        3 => monotonic_vec.push((MonotonicKind::OnStartup, human)),
                        4 => monotonic_vec.push((MonotonicKind::OnUnitActive, human)),
                        5 => monotonic_vec.push((MonotonicKind::OnUnitInactive, human)),
                        _ => continue,
                    }
                }
                state.monotonic_kind = Some(monotonic_vec);
            }

            Kind::Realtime => {
                print!("{}{}", Fg(Blue), style::Bold);
                println!(":: Realtime ::");
                print!("{}", style::Reset);
                loop {
                    print!("{}", Fg(Yellow));
                    print!("{IS_INTERACTIVE}");
                    print!("{}", Fg(Reset));
                    stdout.flush()?;

                    let mut is_interactive = false;
                    stdout.activate_raw_mode()?;
                    loop {
                        let input = keys.next();
                        if let Some(Ok(input)) = input {
                            match input {
                                Key::Char('Y') | Key::Char('y') => {
                                    is_interactive = true;
                                    break;
                                }
                                _ => {
                                    break;
                                }
                            }
                        }
                    }
                    stdout.suspend_raw_mode()?;

                    println!();

                    if is_interactive {
                        let mut format = Format {
                            dow: vec![],
                            year: vec![],
                            month: vec![],
                            day: vec![],
                            time: "".to_string(),
                        };

                        print!("{}", cursor::Show);

                        print!("{}", Fg(Yellow));
                        println!("{DOW}");
                        print!("{}", Fg(Reset));
                        let mut buffer = String::new();
                        stdin.read_line(&mut buffer)?;
                        format.dow = to_dow(buffer);

                        print!("{}", Fg(Yellow));
                        println!("{YEAR}");
                        print!("{}", Fg(Reset));
                        let mut buffer = String::new();
                        stdin.read_line(&mut buffer)?;
                        let mut parsed_year = to_year(buffer.clone());
                        loop {
                            if parsed_year.is_err() {
                                println!("Parse error: Enter again. > ");
                                let mut buffer = String::new();
                                stdin.read_line(&mut buffer)?;
                                parsed_year = to_year(buffer);
                                continue;
                            } else {
                                break;
                            }
                        }
                        format.year = parsed_year?;

                        print!("{}", Fg(Yellow));
                        println!("{MONTH}");
                        print!("{}", Fg(Reset));
                        let mut buffer = String::new();
                        stdin.read_line(&mut buffer)?;
                        let mut parsed_month = to_monthday(buffer.clone());
                        loop {
                            if parsed_month.is_err() {
                                println!("Parse error. Enter again > ");
                                let mut buffer = String::new();
                                stdin.read_line(&mut buffer)?;
                                parsed_month = to_monthday(buffer);
                                continue;
                            } else {
                                break;
                            }
                        }
                        format.month = parsed_month?;

                        print!("{}", Fg(Yellow));
                        println!("{DAY}");
                        print!("{}", Fg(Reset));
                        let mut buffer = String::new();
                        stdin.read_line(&mut buffer)?;
                        let mut parsed_day = to_monthday(buffer.clone());
                        loop {
                            if parsed_day.is_err() {
                                println!("Parse error. Enter again > ");
                                let mut buffer = String::new();
                                stdin.read_line(&mut buffer)?;
                                parsed_day = to_monthday(buffer);
                                continue;
                            } else {
                                break;
                            }
                        }
                        format.day = parsed_day?;

                        print!("{}", Fg(Yellow));
                        println!("{TIME}");
                        print!("{}", Fg(Reset));
                        let mut buffer = String::new();
                        stdin.read_line(&mut buffer)?;
                        let mut parsed_time = to_time(buffer);
                        loop {
                            if parsed_time.is_err() {
                                println!("Parse error. Enter again > ");
                                let mut buffer = String::new();
                                stdin.read_line(&mut buffer)?;
                                parsed_time = to_time(buffer);
                                continue;
                            } else {
                                break;
                            }
                        }
                        format.time = parsed_time?;

                        let formatted = format_to_calendar(format);
                        let output = std::process::Command::new("systemd-analyze")
                            .args(["calendar", &formatted])
                            .output()?
                            .stdout;
                        let output = std::str::from_utf8(&output)?.to_string();
                        if output != *"" {
                            calendar_vec.push(to_normalized(output));
                        } else {
                            println!("Error occured. Please try again.");
                        }
                    } else {
                        print!("{}", cursor::Show);
                        print!("{ENTER_SPEC}");
                        stdout.flush()?;

                        let mut timespec = String::new();

                        loop {
                            let mut buffer = String::new();
                            stdin.read_line(&mut buffer)?;
                            let trimmed = buffer.trim();
                            let output = std::process::Command::new("systemd-analyze")
                                .args(["calendar", trimmed])
                                .output()?
                                .stdout;
                            let mut output = std::str::from_utf8(&output)?.to_string();
                            loop {
                                if output.trim().is_empty() {
                                    print!("{}", Fg(Yellow));
                                    print!("Parse error. Enter again > ");
                                    print!("{}", Fg(Reset));
                                    stdout.flush()?;
                                    let mut buffer = String::new();
                                    stdin.read_line(&mut buffer)?;
                                    let trimmed = buffer.trim();
                                    let re_output = std::process::Command::new("systemd-analyze")
                                        .args(["calendar", trimmed])
                                        .output()?
                                        .stdout;
                                    output = std::str::from_utf8(&re_output)?.to_string();
                                    continue;
                                } else {
                                    break;
                                }
                            }

                            print!("{}", Fg(Green));
                            println!("-------------------------------------------");
                            print!("{output}");
                            println!("-------------------------------------------");
                            print!("{}", Fg(Yellow));
                            print!("{OK_YN}");
                            print!("{}", Fg(Reset));
                            stdout.flush()?;

                            stdout.activate_raw_mode()?;
                            let input = keys.next();
                            if let Some(Ok(key)) = input {
                                match key {
                                    Key::Char('\n') | Key::Char('y') | Key::Char('Y') => {
                                        print!("{}", cursor::Left(100));
                                        println!();
                                        timespec = output.clone();
                                        break;
                                    }
                                    _ => {
                                        print!("{}", cursor::Left(100));
                                        print!("{}", Fg(Yellow));
                                        println!();
                                        print!("{ENTER_AGAIN}");
                                        stdout.suspend_raw_mode()?;
                                        print!("{}", Fg(Reset));
                                        stdout.flush()?;
                                        continue;
                                    }
                                }
                            }
                        }
                        calendar_vec.push(to_normalized(timespec));
                    }
                    print!("{}", Fg(Yellow));
                    print!("Add more calendar? [y/N] ");
                    print!("{}", Fg(Reset));
                    stdout.flush()?;
                    stdout.activate_raw_mode()?;
                    let input = keys.next();
                    if let Some(Ok(key)) = input {
                        match key {
                            Key::Char('y') | Key::Char('Y') => {
                                println!();
                                continue;
                            }
                            _ => {
                                println!();
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    state.calendar = Some(calendar_vec);
    stdout.suspend_raw_mode()?;
    println!();
    print!("{}{}", Fg(Magenta), style::Bold);
    println!("RESULT:");
    println!("++++++++++++++++++++");
    println!("{}", to_timer(state));
    println!("++++++++++++++++++++");
    println!();
    print!("{}{}", Fg(Reset), style::Reset);
    println!("{MORE_DETAIL}");
    print!("{}", cursor::Show);
    Ok(())
}
