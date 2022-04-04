mod errors;
mod functions;
mod messages;
mod run;
mod state;

fn main() -> Result<(), errors::MyError> {
    run::run()?;
    Ok(())
}
