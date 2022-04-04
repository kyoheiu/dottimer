use crate::functions::{to_human, to_normalized, to_timer};

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
    stdout.suspend_raw_mode();
    println!();
    println!("RESULT:");
    println!("{}", to_timer(state));
    print!("{}", cursor::Show);
    Ok(())
}
