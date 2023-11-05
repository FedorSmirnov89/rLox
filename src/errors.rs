use std::error::Error;

pub struct SyntaxError {
    line: u32,
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Syntax error on line {l}.", l = self.line)
    }
}

impl std::fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Syntax error on line {l}.", l = self.line)
    }
}

impl Error for SyntaxError {}
