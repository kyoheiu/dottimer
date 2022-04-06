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
    stdin.read_line(&mut buffer)?;
    state.desciprtion = buffer.trim().to_string();

    println!();

    print!("{}", Fg(Blue));
    println!("{TIMER_KIND}");
    print!("{}", Fg(Reset));

    print!("{}", Fg(Yellow));
    println!("{ENTER_KIND}");
    print!("{}", Fg(Reset));
    print!("{}", cursor::Left(33));

    stdout.activate_raw_mode()?;
    print!("{}", cursor::Hide);
    let mut keys = std::io::stdin().keys();
    let mut kind = Kind::Monotonic;
    println!("> Monotonic");
    print!("{}", cursor::Left(12));
    print!("  Realtime");
    print!("{}{}", cursor::Left(12), cursor::Up(1));
    stdout.flush()?;

    loop {
        let input = keys.next();
        if let Some(Ok(input)) = input {
            match input {
                Key::Up | Key::Down => match kind {
                    Kind::Monotonic => {
                        kind = Kind::Realtime;
                        print!(
                            " {}{}>{}",
                            cursor::Left(1),
                            cursor::Down(1),
                            cursor::Left(1)
                        );
                    }
                    Kind::Realtime => {
                        kind = Kind::Monotonic;
                        print!(" {}{}>{}", cursor::Left(1), cursor::Up(1), cursor::Left(1));
                    }
                },
                Key::Char('\n') => {
                    let (x, y) = stdout.cursor_pos()?;
                    print!("{}", cursor::Goto(x, y + 2));
                    stdout.flush()?;
                    break;
                }
                _ => {
                    continue;
                }
            }
            stdout.flush()?;
        }
    }
    state.timer_kind = kind;
    stdout.suspend_raw_mode()?;
    print!("{}", cursor::Hide);

    println!();
    println!();

    match state.timer_kind {
        Kind::Monotonic => {
            print!("{}", Fg(Yellow));
            println!("{CHOOSE_MONOTONIC_KIND}");
            print!("{}", Fg(Blue));
            print!("{}", cursor::Left(100));
            println!("{KIND1}");
            print!("{}", cursor::Left(100));
            println!("{KIND2}");
            print!("{}", cursor::Left(100));
            println!("{KIND3}");
            print!("{}", cursor::Left(100));
            println!("{KIND4}");
            print!("{}", cursor::Left(100));
            println!("{KIND5}");
            print!("{}", cursor::Left(100));
            print!("{}", Fg(Reset));

            stdout.suspend_raw_mode()?;
            print!("> {}", cursor::Show);
            stdout.flush()?;

            let mut buffer = String::new();
            stdin.read_line(&mut buffer)?;
            let mut chosen = input_to_numvec(buffer.trim().to_string());
            loop {
                if chosen.is_err() {
                    print!("{}", Fg(Yellow));
                    print!("{PARSE_ERROR}");
                    print!("{}", Fg(Reset));
                    stdout.flush()?;
                    let mut buffer = String::new();
                    stdin.read_line(&mut buffer)?;
                    chosen = input_to_numvec(buffer.trim().to_string());
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
                stdout.suspend_raw_mode()?;
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
                                print!("{}", cursor::Left(100));
                                println!();
                                timespan = output.clone();
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

                match i {
                    1 => monotonic_vec.push((MonotonicKind::OnActive, to_human(timespan))),
                    2 => monotonic_vec.push((MonotonicKind::OnBoot, to_human(timespan))),
                    3 => monotonic_vec.push((MonotonicKind::OnStartup, to_human(timespan))),
                    4 => monotonic_vec.push((MonotonicKind::OnUnitActive, to_human(timespan))),
                    5 => monotonic_vec.push((MonotonicKind::OnUnitInactive, to_human(timespan))),
                    _ => continue,
                }
            }
            stdout.activate_raw_mode()?;
            state.monotonic_kind = Some(monotonic_vec);
        }

        Kind::Realtime => {
            print!("{}", cursor::Show);

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
                println!("1. the Day of Week:");
                println!("[Mon, Tue, Wed, Thu, Fri, Sat, Sun]");
                println!("Enter the days you want (i.e. \"Mon, Wed\") default: None");
                print!("{}", Fg(Reset));
                let mut buffer = String::new();
                stdin.read_line(&mut buffer)?;
                format.dow = to_dow(buffer);

                print!("{}", Fg(Yellow));
                println!("2. Year:");
                println!("Enter year (i.e. \"2022\", \"2023..2025\", or \"2024 2025 2028..2030\") default: *");
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
                println!("3. Month:");
                println!("Enter month (i.e. \"1\", \"3..5\", or \"2 4 6..11\") default: *");
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
                println!("4. Day:");
                println!("Enter day (i.e. \"2\", \"13..15\", or \"20 24 26..28\") default: *");
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
                println!("5. Time:");
                println!("Enter time (i.e. \"12:00:00\") default: 00:00:00");
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
                    state.calendar = Some(to_normalized(output));
                } else {
                    println!("Error occured. Please try again.");
                }
            } else {
                stdout.suspend_raw_mode()?;
                print!("{}", cursor::Show);
                print!("Enter the time spec > ");
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
                    print!("OK? [Y/n] ");
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
                                print!("Enter again > ");
                                stdout.suspend_raw_mode()?;
                                print!("{}", Fg(Reset));
                                stdout.flush()?;
                                continue;
                            }
                        }
                    }
                }
                state.calendar = Some(to_normalized(timespec));
            }
        }
    }
    stdout.suspend_raw_mode()?;
    println!();
    print!("{}{}", Fg(Magenta), style::Bold);
    println!("RESULT:");
    println!("++++++++++++++++++++");
    println!("{}", to_timer(state));
    println!("++++++++++++++++++++");
    println!();
    print!("{}{}", Fg(Reset), style::Reset);
    println!("For more details, see systemd.timer(5) and systemd.time(7).");
    print!("{}", cursor::Show);
    Ok(())
}
