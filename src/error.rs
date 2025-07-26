use std::fmt;

/// `WordError` represents specific errors that can occur when creating, validating, or manipulating a `Word`.
/// These errors typically arise from invalid input or attempts to create words that do not conform to expected rules.
#[derive(Debug)]
pub enum WordError {
    /// Indicates that a word segment (prefix, crossed character, or suffix) is empty or contains only whitespace.
    EmptyOrWhitespaceSegment,
    /// Indicates that a word segment contains lowercase characters, which are not allowed.
    LowercaseCharactersInSegment,
}

impl fmt::Display for WordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WordError::EmptyOrWhitespaceSegment => {
                write!(f, "Segment cannot be empty or contain only whitespace.")
            }
            WordError::LowercaseCharactersInSegment => {
                write!(f, "Segment cannot contain lowercase characters.")
            }
        }
    }
}

impl std::error::Error for WordError {}

/// `GridError` represents errors that can occur during operations on the crossword `Grid`.
/// These errors typically relate to invalid directions, word placement issues, or underlying `WordError`s.
#[derive(Debug)]
pub enum GridError {
    /// Indicates that an invalid or unsupported direction was provided for a grid operation.
    InvalidDirection(String),
    /// Wraps a `WordError` that occurred during a grid operation, providing more context.
    WordError(WordError),
}

impl fmt::Display for GridError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GridError::InvalidDirection(msg) => write!(f, "Invalid direction: {msg}"),
            GridError::WordError(e) => write!(f, "Word error: {e}"),
        }
    }
}

impl std::error::Error for GridError {}

impl From<WordError> for GridError {
    fn from(err: WordError) -> Self {
        GridError::WordError(err)
    }
}

/// `Error` represents general application errors, encompassing `WordError` and `GridError`,
/// as well as custom error messages.
#[derive(Debug)]
pub enum Error {
    /// Wraps a `WordError` that occurred within the application.
    WordError(WordError),
    /// Wraps a `GridError` that occurred within the application.
    GridError(GridError),
    /// Represents a custom error message, useful for general application-level failures.
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::WordError(e) => write!(f, "Word error: {e}"),
            Error::GridError(e) => write!(f, "Grid error: {e}"),
            Error::Custom(msg) => write!(f, "Application error: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<WordError> for Error {
    fn from(err: WordError) -> Self {
        Error::WordError(err)
    }
}

impl From<GridError> for Error {
    fn from(err: GridError) -> Self {
        Error::GridError(err)
    }
}

