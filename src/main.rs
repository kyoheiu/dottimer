mod errors;
mod functions;
mod run;
mod state;

fn main() -> Result<(), errors::MyError> {
    run::run()?;
    Ok(())
}
