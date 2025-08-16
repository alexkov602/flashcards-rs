use std::io;

mod cli;
mod flashcard;
mod storage;

fn main() -> Result<(), io::Error> {
    cli::run()
}
