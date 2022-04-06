mod errors;
mod functions;
mod messages;
mod run;
mod state;

fn main() -> Result<(), errors::MyError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        if args[1] == "-o" {
            run::run(true)?;
        } else if (args[1] == "-H") | (args[1] == "--help") {
            println!("{}", messages::HELP);
        }
    } else {
        run::run(false)?;
    }
    Ok(())
}
