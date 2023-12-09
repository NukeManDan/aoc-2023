use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error("Line must start with \"Game <ID>:\" where <ID> is a uint")]
    #[diagnostic(code(aoc::parsing_error))]
    MissingId,

    #[error("Game must have three sets")]
    #[diagnostic(code(aoc::parsing_error))]
    MissingSet,

    #[error("Set must have form \" <NUMBER> <color>\" where <color> is a [red | green | blue] and <NUMBER> is a uint")]
    #[diagnostic(code(aoc::parsing_error))]
    SetMalformed,

    #[error("Cannot sum numbers")]
    #[diagnostic(code(aoc::processing_error))]
    SumError,
}
