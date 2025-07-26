use crate::error::WordError;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// `Direction` defines the possible orientations for a word within the crossword puzzle grid.
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum Direction {
    /// Represents a horizontal orientation, where the word extends from left to right.
    Horizontal,
    /// Represents a vertical orientation, where the word extends from top to bottom.
    Vertical,
    /// Represents an unset or undefined direction. This is typically used as a default
    /// or an initial state before a direction is explicitly assigned.
    #[default]
    NotSet,
}

/// `Position` represents the (x, y) coordinates of a cell on the crossword grid.
/// `x` corresponds to the column index, and `y` corresponds to the row index.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    /// The x-coordinate (column index) of the position.
    pub x: usize,
    /// The y-coordinate (row index) of the position.
    pub y: usize,
}

#[derive(Debug, Default, Clone)]
/// `Segment` represents a part of a word, typically used when a word is broken down
/// by a crossing character. It consists of a `prefix`, the `crossed` character itself,
/// and a `suffix`.
pub struct Segment<'a> {
    /// The part of the word that comes before the `crossed` character.
    pub prefix: &'a str,
    /// The single character at the crossing point, which connects to another word.
    pub crossed: char,
    /// The part of the word that comes after the `crossed` character.
    pub suffix: &'a str,
}

impl<'a> Segment<'a> {
    /// Creates a new `Segment` instance.
    ///
    /// This function validates the input to ensure that segments are not empty or whitespace-only,
    /// and that they do not contain lowercase characters.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The string slice for the prefix part of the word.
    /// * `crossed` - The character at the crossing point.
    /// * `suffix` - The string slice for the suffix part of the word.
    ///
    /// # Returns
    ///
    /// - `Ok(Segment)` if the segment is valid.
    /// - `Err(WordError)` if the segment is invalid (e.g., empty, contains whitespace, or lowercase characters).
    ///
    /// # Errors
    ///
    /// Returns a `WordError::EmptyOrWhitespaceSegment` if `prefix`, `suffix` are empty and `crossed` is whitespace.
    /// Returns a `WordError::LowercaseCharactersInSegment` if any part contains lowercase characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::word::Segment;
    ///
    /// let segment = Segment::new("APP", 'L', "E").unwrap();
    /// assert_eq!(segment.prefix, "APP");
    /// assert_eq!(segment.crossed, 'L');
    /// assert_eq!(segment.suffix, "E");
    ///
    /// // Example of an error
    /// let error_segment = Segment::new("", ' ', "");
    /// assert!(error_segment.is_err());
    /// ```
    pub fn new(prefix: &'a str, crossed: char, suffix: &'a str) -> Result<Self, WordError> {
        if prefix.is_empty() && suffix.is_empty() && crossed.is_ascii_whitespace() {
            return Err(WordError::EmptyOrWhitespaceSegment);
        }

        if prefix.chars().any(|c| c.is_lowercase())
            || crossed.is_lowercase()
            || suffix.chars().any(|c| c.is_lowercase())
        {
            return Err(WordError::LowercaseCharactersInSegment);
        }

        Ok(Segment {
            prefix,
            crossed,
            suffix,
        })
    }
}

impl Segment<'_> {
    /// Returns the full word by combining the `prefix`, `crossed` character, and `suffix`.
    /// The result is a `Vec<char>` representing the complete word.
    ///
    /// # Returns
    ///
    /// A `Vec<char>` containing all characters of the word.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::word::Segment;
    ///
    /// let segment = Segment::new("APP", 'L', "E").unwrap();
    /// assert_eq!(segment.full_word(), vec!['A', 'P', 'P', 'L', 'E']);
    /// ```
    pub fn full_word(&self) -> Vec<char> {
        self.prefix
            .chars()
            .chain(std::iter::once(self.crossed))
            .chain(self.suffix.chars())
            .collect()
    }

    /// Returns the full word as a `String`.
    ///
    /// This function combines the `prefix`, `crossed` character, and `suffix` into a single `String`.
    ///
    /// # Returns
    ///
    /// A `String` containing the complete word.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::word::Segment;
    ///
    /// let segment = Segment::new("APP", 'L', "E").unwrap();
    /// assert_eq!(segment.full_word_str(), "APPLE");
    /// ```
    pub fn full_word_str(&self) -> String {
        let mut full_word = String::new();
        for ch in self.full_word() {
            full_word.push(ch);
        }
        full_word
    }
}

#[derive(Debug, Default, Clone)]
/// `Word` represents a word intended for placement in the crossword puzzle.
/// It encapsulates the word's content (`Segment`), its `Position` on the grid,
/// its calculated `origin` (start of the word), and its `Direction`.
pub struct Word<'a> {
    /// The segment of the word.
    pub segment: Segment<'a>,
    /// The position of the crossed character on the grid.
    pub position: Position,
    /// The calculated origin of the word based on its position and direction.
    pub origin: Position,
    /// The direction of the word (horizontal or vertical).
    pub direction: Direction,
}

impl<'a> Word<'a> {
    /// Creates a new `Word` instance from its constituent parts.
    ///
    /// This function validates the `prefix`, `crossed` character, and `suffix`
    /// by creating a `Segment`.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The string slice for the prefix part of the word.
    /// * `crossed` - The character at the crossing point.
    /// * `suffix` - The string slice for the suffix part of the word.
    ///
    /// # Returns
    ///
    /// - `Ok(Word)` if the word is valid.
    /// - `Err(WordError)` if the underlying `Segment` creation fails.
    ///
    /// # Errors
    ///
    /// Returns a `WordError` if `Segment::new` returns an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::word::Word;
    ///
    /// let word = Word::value("APP", 'L', "E").unwrap();
    /// assert_eq!(word.segment.full_word_str(), "APPLE");
    /// ```
    pub fn value(prefix: &'a str, crossed: char, suffix: &'a str) -> Result<Self, WordError> {
        Ok(Self {
            segment: Segment::new(prefix, crossed, suffix)?,
            ..Default::default()
        })
    }

    /// Sets the `Position` (x, y) of the word's `crossed` character on the grid.
    ///
    /// This is a builder-pattern method, returning `self` for chaining.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate (column) of the crossed character.
    /// * `y` - The y-coordinate (row) of the crossed character.
    ///
    /// # Returns
    ///
    /// The `Word` instance with its `position` updated.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::word::{Word, Position};
    ///
    /// let word = Word::value("", 'A', "").unwrap().position(5, 10);
    /// assert_eq!(word.position, Position { x: 5, y: 10 });
    /// ```
    pub fn position(mut self, x: usize, y: usize) -> Self {
        self.position = Position { x, y };
        self
    }

    /// Sets the `Direction` of the word.
    ///
    /// This is a builder-pattern method, returning `self` for chaining.
    ///
    /// # Arguments
    ///
    /// * `direction` - The `Direction` (Horizontal or Vertical) for the word.
    ///
    /// # Returns
    ///
    /// The `Word` instance with its `direction` updated.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::word::{Word, Direction};
    ///
    /// let word = Word::value("", 'A', "").unwrap().direction(Direction::Vertical);
    /// assert_eq!(word.direction, Direction::Vertical);
    /// ```
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Updates the `origin` of the word based on its `position`, `direction`, and `prefix` length.
    ///
    /// The `origin` represents the `Position` of the very first character of the word on the grid.
    /// This is crucial for correctly placing the word on the `Grid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::word::{Word, Direction, Position};
    ///
    /// let mut word = Word::value("APP", 'L', "E").unwrap().position(3, 0).direction(Direction::Horizontal);
    /// word.update_position();
    /// // 'L' is at x=3, prefix "APP" has length 3, so origin.x should be 3 - 3 = 0
    /// assert_eq!(word.origin, Position { x: 0, y: 0 });
    /// ```
    pub fn update_position(&mut self) {
        match self.direction {
            Direction::Vertical => {
                self.origin.x = self.position.x;
                self.origin.y = self.position.y.saturating_sub(self.segment.prefix.len())
            }
            Direction::Horizontal => {
                self.origin.x = self.position.x.saturating_sub(self.segment.prefix.len());
                self.origin.y = self.position.y;
            }
            _ => {}
        }
    }

    /// Returns a `Vec` of all `Position`s occupied by the word on the grid.
    ///
    /// This calculates the coordinates for each character of the word based on its
    /// `origin` and `direction`.
    ///
    /// # Returns
    ///
    /// A `Vec<Position>` containing the coordinates for each character of the word.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::word::{Word, Direction, Position};
    ///
    /// let mut word = Word::value("", 'A', "BC").unwrap().position(0, 0).direction(Direction::Horizontal);
    /// word.update_position();
    /// let positions = word.positions();
    /// assert_eq!(positions.len(), 3);
    /// assert_eq!(positions[0], Position { x: 0, y: 0 });
    /// assert_eq!(positions[1], Position { x: 1, y: 0 });
    /// assert_eq!(positions[2], Position { x: 2, y: 0 });
    /// ```
    pub fn positions(&self) -> Vec<Position> {
        let length = self.segment.full_word().len();
        match self.direction {
            Direction::Horizontal => (self.origin.x..self.origin.x + length)
                .map(|x| Position {
                    x,
                    y: self.origin.y,
                })
                .collect(),
            Direction::Vertical => (self.origin.y..self.origin.y + length)
                .map(|y| Position {
                    x: self.origin.x,
                    y,
                })
                .collect(),

            _ => vec![],
        }
    }
}
