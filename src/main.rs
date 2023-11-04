use anyhow::Result;
use clap::Parser;
use rlox::{interpret_lox_file, run_prompt, Arguments};

fn main() -> Result<()> {
    let arguments = Arguments::parse();

    match arguments.mode() {
        rlox::Mode::Prompt => run_prompt(),
        rlox::Mode::File {
            file_path: lox_string,
        } => interpret_lox_file(&lox_string),
    }
}
