
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error("No more lines in input to parse")]
    #[diagnostic(code(aoc::parsing_error))]
    EndInput,

    #[error("input unable to parse")]
    #[diagnostic(code(aoc::parsing_error))]
    BadInput,

    #[error("unable to get slice in string")]
    #[diagnostic(code(aoc::parsing_error))]
    CannotIndex,
}
