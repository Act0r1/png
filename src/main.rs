use clap::Parser;
use Cli;
use crate::commands::execute_command;
mod error;
/// Holds any kind of error.
pub type Error = Box<dyn std::error::Error>;
/// Holds a `Result` of any kind of error.
pub type Result<T> = std::result::Result<T, Error>;
fn main() {
    let args = Cli::parse();
    match execute_command(args.command) {
        Ok(()) => println!("Worked successfully."),
        Err(why) => println!("{}", why),
    }
}
