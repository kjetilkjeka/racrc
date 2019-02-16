//! Everything related to compile errors
//! 
//! Contains:
//!  - Every possible error definition and kind/code possible to get during compilation
//!  - Definition of the error table and how it is to be formated

#[derive(Debug, PartialEq)]
pub(crate) struct Error {
    kind: ErrorKind,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ErrorKind {
    RedefinedSymbol,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Error {
            kind,
        }
    }
}

