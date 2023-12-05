use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error("Expected a number on this line")]
    #[diagnostic(code(aoc::numeric_error))]
    DigitMissing,

    #[error("Cannot sum numbers")]
    #[diagnostic(code(aoc::processing_errror))]
    SumError,
}
