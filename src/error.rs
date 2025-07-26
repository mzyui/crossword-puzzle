use std::fmt;

#[derive(Debug)]
/// Represents errors that can occur when creating or manipulating a `Word`.
pub enum WordError {
    EmptyOrWhitespaceSegment,
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

#[derive(Debug)]
/// Represents errors that can occur during grid operations.
pub enum GridError {
    InvalidDirection(String),
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

#[derive(Debug)]
/// Represents general application errors, encompassing `WordError` and `GridError`.
pub enum Error {
    WordError(WordError),
    GridError(GridError),
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

