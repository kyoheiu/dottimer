use crate::functions::*;

use super::errors::MyError;
use super::functions::input_to_numvec;
use super::messages::KINDS_OF_TIMER;
use super::state::*;
use std::io::Write;
use std::io::{stdin, stdout};
use termion::clear::CurrentLine;
use termion::cursor::{self, DetectCursorPos};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn run() -> Result<(), MyError> {
    let mut state = State::new();
    let mut stdout = stdout().into_raw_mode().unwrap();
    print!("{}", cursor::Hide);

    stdout.suspend_raw_mode()?;
    print!("description > {}", cursor::Show);
    stdout.flush()?;

    let mut buffer = String::new();
    let stdin_ontype = std::io::stdin();
    stdin_ontype.read_line(&mut buffer)?;
    state.desciprtion = buffer.trim().to_string();

    print!("{}", cursor::Hide);
    stdout.activate_raw_mode()?;
    println!();

    let (_, y) = stdout.cursor_pos()?;
    let mut i = y;
    for line in KINDS_OF_TIMER.lines() {
        println!("{}{}", line, cursor::Goto(1, i));
        i += 1;
    }

    println!("Which kind of timer do you want?");
    print!("{}", cursor::Left(33));

    let mut stdin = stdin().keys();
    let mut kind = Kind::Monotonic;
    println!("> Monotonic");
    print!("{}", cursor::Left(12));
    print!("  Realtime");
    print!("{}", cursor::Left(12));
    stdout.flush()?;

    loop {
        let input = stdin.next();
        if let Some(Ok(input)) = input {
            match input {
                Key::Up | Key::Down => match kind {
                    Kind::Monotonic => {
                        kind = Kind::Realtime;
                        print!("{}{}{}", CurrentLine, cursor::Up(1), CurrentLine);
                        println!("  Monotonic");
                        print!("{}", cursor::Left(12));
                        print!("> Realtime");
                        print!("{}", cursor::Left(12));
                    }
                    Kind::Realtime => {
                        kind = Kind::Monotonic;
                        print!("{}{}{}", CurrentLine, cursor::Up(1), CurrentLine);
                        println!("> Monotonic");
                        print!("{}", cursor::Left(12));
                        print!("  Realtime");
                        print!("{}", cursor::Left(12));
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

    println!();
    println!();

    match state.timer_kind {
        Kind::Monotonic => {
            println!("Choose kinds of Monotonic timer(i.e. 1 | 2 3 4):");
            print!("{}", cursor::Left(100));
            println!("1 OnActiveSec: relative to the mooment the timer unit is activated.");
            print!("{}", cursor::Left(100));
            println!("2 OnBootSec: Relative to when the machines was booted up.");
            print!("{}", cursor::Left(100));
            println!("3 OnStartupSec: Relative to when the service manager was first started.");
            print!("{}", cursor::Left(100));
            println!(
                "4 OnUnitActiveSec: Relative to when the unit is activating was last activated."
            );
            print!("{}", cursor::Left(100));
            println!(
                "5 OnUnitInactiveSec: Relative to when the unit is activating was last deactivated."
            );
            print!("{}", cursor::Left(100));

            stdout.suspend_raw_mode()?;
            print!("> {}", cursor::Show);
            stdout.flush()?;

            let mut buffer = String::new();
            let stdin_ontype = std::io::stdin();
            stdin_ontype.read_line(&mut buffer)?;
            let chosen = input_to_numvec(buffer)?;

            let mut monotonic_vec: Vec<(MonotonicKind, String)> = vec![];
            let mut timespan = String::new();

            for i in chosen {
                stdout.suspend_raw_mode()?;
                match i {
                    1 => print!("OnActiveSec > "),
                    2 => print!("OnBootSec > "),
                    3 => print!("OnStartupSec > "),
                    4 => print!("OnUnitActiveSec > "),
                    5 => print!("OnUnitInactiveSec > "),
                    _ => continue,
                }
                stdout.flush()?;
                let mut buffer = String::new();
                let stdin_monotonic_timer = std::io::stdin();
                stdin_monotonic_timer.read_line(&mut buffer)?;
                let trimmed = buffer.trim();
                let output = std::process::Command::new("systemd-analyze")
                    .args(["timespan", trimmed])
                    .output()?
                    .stdout;
                let output = std::str::from_utf8(&output)?.to_string();
                timespan = output.clone();

                print!("{output}");
                print!("Is this OK? [Y/n] ");
                stdout.flush()?;
                stdout.activate_raw_mode()?;

                loop {
                    let input = stdin.next();
                    if let Some(Ok(input)) = input {
                        match input {
                            Key::Char('\n') => {
                                print!("{}", cursor::Left(100));
                                println!();
                                break;
                            }
                            _ => {
                                print!("{}", cursor::Left(100));
                                println!();
                                stdout.suspend_raw_mode()?;
                                match i {
                                    1 => print!("OnActiveSec > "),
                                    2 => print!("OnBootSec > "),
                                    3 => print!("OnStartupSec > "),
                                    4 => print!("OnUnitActiveSec > "),
                                    5 => print!("OnUnitInactiveSec > "),
                                    _ => continue,
                                }
                                stdout.flush()?;
                                let mut buffer = String::new();
                                let stdin_monotonic_timer = std::io::stdin();
                                stdin_monotonic_timer.read_line(&mut buffer)?;
                                let output = std::process::Command::new("systemd-analyze")
                                    .args(["timespan", &buffer])
                                    .output()?
                                    .stdout;
                                let output = std::str::from_utf8(&output)?.to_string();
                                timespan = output.clone();

                                print!("{output}");
                                print!("Is this OK? [Y/n] ");
                                stdout.flush()?;
                                stdout.activate_raw_mode()?;
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
            println!("Do you want the interactive input for time spec? [y/N]");
            print!("{}", cursor::Left(100));
            stdout.flush()?;

            let mut is_interactive = false;
            loop {
                let input = stdin.next();
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

            if is_interactive {
                let mut format = Format {
                    dow: vec![],
                    year: vec![],
                    month: vec![],
                    day: vec![],
                    time: "".to_string(),
                };

                stdout.suspend_raw_mode()?;
                print!("{}", cursor::Show);
                println!("1. the Day of Week:");
                println!("[Mon, Tue, Wed, Thu, Fri, Sat, Sun]");
                println!("Enter one or more of the day you want, with whitespace split:");
                let mut buffer = String::new();
                let stdin_dow = std::io::stdin();
                stdin_dow.read_line(&mut buffer)?;
                format.dow = to_dow(buffer);

                println!("2. Year:");
                println!("Enter one or more year (i.e. \"2022\", \"2023..2025\", or \"2024 2025 2028..2030\") default: *");
                let mut buffer = String::new();
                let stdin_year = std::io::stdin();
                stdin_year.read_line(&mut buffer)?;
                format.year = to_year(buffer)?;

                println!("3. Month:");
                println!(
                    "Enter one or more month (i.e. \"1\", \"3..5\", or \"2 4 6..11\") default: *"
                );
                let mut buffer = String::new();
                let stdin_month = std::io::stdin();
                stdin_month.read_line(&mut buffer)?;
                format.month = to_monthday(buffer)?;

                println!("4. Day:");
                println!("Enter one or more day (i.e. \"2\", \"13..15\", or \"20 24 26..28\") default: *");
                let mut buffer = String::new();
                let stdin_day = std::io::stdin();
                stdin_day.read_line(&mut buffer)?;
                format.day = to_monthday(buffer)?;

                println!("5. Time:");
                println!("Enter time (i.e. \"12:00:00\") default: 00:00:00");
                let mut buffer = String::new();
                let stdin_time = std::io::stdin();
                stdin_time.read_line(&mut buffer)?;
                format.time = to_time(buffer).unwrap();

                print!("{:?}", format);

                let formatted = format_to_calendar(format);
                let output = std::process::Command::new("systemd-analyze")
                    .args(["calendar", &formatted])
                    .output()?
                    .stdout;
                let output = std::str::from_utf8(&output)?.to_string();
                if output != *"" {
                    state.calendar = Some(to_normalized(output));
                }
            } else {
                print!("Enter the time spec > ");

                stdout.suspend_raw_mode()?;
                print!("{}", cursor::Show);
                stdout.flush()?;

                let mut timespec = String::new();
                let mut buffer = String::new();
                let stdin_calender = std::io::stdin();
                stdin_calender.read_line(&mut buffer)?;
                let trimmed = buffer.trim();
                let output = std::process::Command::new("systemd-analyze")
                    .args(["calendar", trimmed])
                    .output()?
                    .stdout;
                let output = std::str::from_utf8(&output)?.to_string();
                timespec = output.clone();

                print!("{output}");
                print!("Is this OK? [Y/n] ");
                stdout.flush()?;
                stdout.activate_raw_mode()?;

                loop {
                    let input = stdin.next();
                    if let Some(Ok(input)) = input {
                        match input {
                            Key::Char('\n') => {
                                print!("{}", cursor::Left(100));
                                println!();
                                break;
                            }
                            _ => {
                                print!("{}", cursor::Left(100));
                                println!();
                                print!("Enter the time spec > ");

                                stdout.suspend_raw_mode()?;
                                print!("{}", cursor::Show);
                                stdout.flush()?;

                                let mut buffer = String::new();
                                let stdin_calender = std::io::stdin();
                                stdin_calender.read_line(&mut buffer)?;
                                let trimmed = buffer.trim();
                                let output = std::process::Command::new("systemd-analyze")
                                    .args(["calendar", trimmed])
                                    .output()?
                                    .stdout;
                                let output = std::str::from_utf8(&output)?.to_string();
                                timespec = output.clone();

                                print!("{output}");
                                print!("Is this OK? [Y/n] ");
                                stdout.flush()?;
                                stdout.activate_raw_mode()?;
                            }
                        }
                    }
                }

                state.calendar = Some(to_normalized(timespec));
            }
        }
    }
    stdout.suspend_raw_mode();
    println!();
    println!("RESULT:");
    println!("{}", to_timer(state));
    print!("{}", cursor::Show);
    Ok(())
}
