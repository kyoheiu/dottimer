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
            print!("> {}", cursor::Show);
            stdout.flush()?;

            stdout.suspend_raw_mode()?;
            let mut buffer = String::new();
            let stdin_ontype = std::io::stdin();
            stdin_ontype.read_line(&mut buffer)?;
            let chosen = input_to_numvec(buffer)?;

            let mut monotonic_vec: Vec<(MonotonicKind, String)> = vec![];
            for i in chosen {
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
                    .output()?;
                let output = std::str::from_utf8(&output.stdout)?;
                print!("{output}");
                stdout.flush()?;
                match i {
                    1 => monotonic_vec.push((MonotonicKind::OnActive, buffer)),
                    2 => monotonic_vec.push((MonotonicKind::OnBoot, buffer)),
                    3 => monotonic_vec.push((MonotonicKind::OnStartup, buffer)),
                    4 => monotonic_vec.push((MonotonicKind::OnUnitActive, buffer)),
                    5 => monotonic_vec.push((MonotonicKind::OnUnitInactive, buffer)),
                    _ => continue,
                }
            }
            println!("{:?}", monotonic_vec);
            stdout.activate_raw_mode()?;
        }
        Kind::Realtime => {}
    }
    print!("{}", cursor::Show);
    Ok(())
}
