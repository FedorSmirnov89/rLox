use clap::Subcommand;

#[derive(clap::Parser)]
pub struct Arguments {
    /// The mode to start the interpreter in
    #[command(subcommand)]
    mode: Mode,
}

impl Arguments {
    pub fn mode(&self) -> &Mode {
        &self.mode
    }
}

#[derive(Subcommand)]
pub enum Mode {
    /// Interprets the code provided via a prompt, one line at a time
    Prompt,
    /// Interprets the code provided via a file
    File {
        /// The string of lox source code
        file_path: String,
    },
}
