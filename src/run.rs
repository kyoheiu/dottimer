use crate::functions::*;

use super::errors::MyError;
use super::functions::input_to_numvec;
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
    print!("{}description > {}{}", Fg(Yellow), cursor::Show, Fg(Reset));
    stdout.flush()?;

    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;
    state.desciprtion = buffer.trim().to_string();

    println!();

    print!("{}", Fg(Blue));
    println!(
        "systemd timers are difined as one of two types:\nMonotonic timers activate after a time span relative to a varying starting point.\nRealtime timers activate on a calendar event, the same way that cronjobs do."
    );
    print!("{}", Fg(Reset));

    print!("{}", Fg(Yellow));
    println!("Which kind of timer do you want?");
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
            println!("Choose kinds of Monotonic timer(i.e. \"1\" or \"2 3 4\"):");
            print!("{}", Fg(Blue));
            print!("{}", cursor::Left(100));
            println!("1) OnActiveSec: Relative to the mooment the timer unit is activated.");
            print!("{}", cursor::Left(100));
            println!("2) OnBootSec: Relative to when the machines was booted up.");
            print!("{}", cursor::Left(100));
            println!("3) OnStartupSec: Relative to when the service manager was first started.");
            print!("{}", cursor::Left(100));
            println!(
                "4) OnUnitActiveSec: Relative to when the unit is activating was last activated."
            );
            print!("{}", cursor::Left(100));
            println!(
                "5) OnUnitInactiveSec: Relative to when the unit is activating was last deactivated."
            );
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
                    print!("Parse error. Enter again > ");
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
            println!("\nEnter the time span for each timer.\nExample: \"50\" for OnBootSec means 50s after boot-up. \nThe argument may also include time units.\nAnother example: \"5h 30min\" for OnBootSec means 5 hours and 30 minutes after boot-up.\nFor details about the syntax of time spans, see systemd.time(7).");
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
                            print!("Parse error. Enter again > ");
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
                                timespan = output.clone();
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
            println!("Do you want the interactive input for time spec? [y/N]");
            print!("{}", cursor::Left(100));
            print!("{}", Fg(Reset));
            stdout.flush()?;

            let mut is_interactive = false;
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

            println!();

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
                let mut parsed = to_year(buffer.clone());
                loop {
                    if parsed.is_err() {
                        println!("Parse error. Enter again > ");
                        let mut buffer = String::new();
                        stdin.read_line(&mut buffer)?;
                        parsed = to_year(buffer);
                        continue;
                    } else {
                        break;
                    }
                }
                format.year = parsed?;

                print!("{}", Fg(Yellow));
                println!("3. Month:");
                println!("Enter month (i.e. \"1\", \"3..5\", or \"2 4 6..11\") default: *");
                print!("{}", Fg(Reset));
                let mut buffer = String::new();
                stdin.read_line(&mut buffer)?;
                format.month = to_monthday(buffer)?;

                print!("{}", Fg(Yellow));
                println!("4. Day:");
                println!("Enter day (i.e. \"2\", \"13..15\", or \"20 24 26..28\") default: *");
                print!("{}", Fg(Reset));
                let mut buffer = String::new();
                stdin.read_line(&mut buffer)?;
                format.day = to_monthday(buffer)?;

                print!("{}", Fg(Yellow));
                println!("5. Time:");
                println!("Enter time (i.e. \"12:00:00\") default: 00:00:00");
                print!("{}", Fg(Reset));
                let mut buffer = String::new();
                stdin.read_line(&mut buffer)?;
                format.time = to_time(buffer).unwrap();

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
                stdout.suspend_raw_mode()?;
                print!("{}", cursor::Show);
                print!("Enter the time spec > ");

                stdout.flush()?;

                let mut timespec = String::new();
                let mut buffer = String::new();
                stdin.read_line(&mut buffer)?;
                let trimmed = buffer.trim();
                let output = std::process::Command::new("systemd-analyze")
                    .args(["calendar", trimmed])
                    .output()?
                    .stdout;
                let output = std::str::from_utf8(&output)?.to_string();
                timespec = output;

                stdout.flush()?;
                stdout.activate_raw_mode()?;

                loop {
                    let input = keys.next();
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
                                stdin.read_line(&mut buffer)?;
                                let trimmed = buffer.trim();
                                let output = std::process::Command::new("systemd-analyze")
                                    .args(["calendar", trimmed])
                                    .output()?
                                    .stdout;
                                let output = std::str::from_utf8(&output)?.to_string();
                                timespec = output.clone();

                                print!("{output}");
                                print!("OK? [Y/n] ");
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
    stdout.suspend_raw_mode()?;
    println!();
    print!("{}{}", Fg(Magenta), style::Bold);
    println!("RESULT:");
    println!("++++++++++++++++++++");
    println!("{}", to_timer(state));
    println!("++++++++++++++++++++");
    println!();
    print!("{}{}", Fg(Reset), style::Reset);
    println!("For more details, see systemd.timer(5) and systemd.timer(7).");
    print!("{}", cursor::Show);
    Ok(())
}
