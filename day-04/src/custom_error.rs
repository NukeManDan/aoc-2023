use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error("unable to get slice in string")]
    #[diagnostic(code(aoc::parsing_error))]
    CannotIndex,

    #[error("unable parse digit string")]
    #[diagnostic(code(aoc::parsing_error))]
    CannotParse,
}
