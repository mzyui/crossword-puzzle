//! Defines the error types used throughout the crossword puzzle generator.
//! This module provides a structured way to handle various issues that can arise
//! during word processing, grid generation, and other application-level operations.

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

/// Implements the `Display` trait for `WordError`, allowing errors to be formatted as user-friendly strings.
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

/// Implements the `Error` trait for `WordError`, providing a common interface for error handling.
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

/// Implements the `Display` trait for `GridError`, allowing errors to be formatted as user-friendly strings.
impl fmt::Display for GridError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GridError::InvalidDirection(msg) => write!(f, "Invalid direction: {msg}"),
            GridError::WordError(e) => write!(f, "Word error: {e}"),
        }
    }
}

/// Implements the `Error` trait for `GridError`, providing a common interface for error handling.
impl std::error::Error for GridError {}

/// Implements conversion from `WordError` to `GridError`.
/// This allows `WordError`s to be easily wrapped within `GridError`s.
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

/// Implements the `Display` trait for `Error`, allowing general application errors to be formatted as user-friendly strings.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::WordError(e) => write!(f, "Word error: {e}"),
            Error::GridError(e) => write!(f, "Grid error: {e}"),
            Error::Custom(msg) => write!(f, "Application error: {msg}"),
        }
    }
}

/// Implements the `Error` trait for `Error`, providing a common interface for general application error handling.
impl std::error::Error for Error {}

/// Implements conversion from `WordError` to `Error`.
/// This allows `WordError`s to be easily wrapped within the top-level `Error` type.
impl From<WordError> for Error {
    fn from(err: WordError) -> Self {
        Error::WordError(err)
    }
}

/// Implements conversion from `GridError` to `Error`.
/// This allows `GridError`s to be easily wrapped within the top-level `Error` type.
impl From<GridError> for Error {
    fn from(err: GridError) -> Self {
        Error::GridError(err)
    }
}

