use crate::error::WordError;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Represents the direction of a word in the crossword puzzle.
pub enum Direction {
    /// Horizontal direction (left to right).
    Horizontal,
    /// Vertical direction (top to bottom).
    Vertical,
    /// Direction not yet set.
    #[default]
    NotSet,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
/// Represents the (x, y) coordinates of a position on the crossword grid.
pub struct Position {
    /// The x-coordinate (column).
    pub x: usize,
    /// The y-coordinate (row).
    pub y: usize,
}

#[derive(Debug, Default, Clone)]
/// Represents a segment of a word, typically used for words that cross another word.
pub struct Segment<'a> {
    /// The part of the word before the crossed character.
    pub prefix: &'a str,
    /// The character that crosses another word.
    pub crossed: char,
    /// The part of the word after the crossed character.
    pub suffix: &'a str,
}

impl<'a> Segment<'a> {
    /// Creates a new `Segment`.
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
    /// Returns the full word by combining the prefix, crossed character, and suffix.
    pub fn full_word(&self) -> Vec<char> {
        self.prefix
            .chars()
            .chain(std::iter::once(self.crossed))
            .chain(self.suffix.chars())
            .collect()
    }

    /// Returns the full word as String
    pub fn full_word_str(&self) -> String {
        let mut full_word = String::new();
        for ch in self.full_word() {
            full_word.push(ch);
        }
        full_word
    }
}

#[derive(Debug, Default, Clone)]
/// Represents a word in the crossword puzzle, including its segment, position, and direction.
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
    /// Creates a new `Word` with the given prefix, crossed character, and suffix.
    pub fn value(prefix: &'a str, crossed: char, suffix: &'a str) -> Result<Self, WordError> {
        Ok(Self {
            segment: Segment::new(prefix, crossed, suffix)?,
            ..Default::default()
        })
    }

    /// Sets the position of the word (x, y).
    pub fn position(mut self, x: usize, y: usize) -> Self {
        self.position = Position { x, y };
        self
    }

    /// Sets the direction of the word.
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Updates the origin of the word based on its position, direction, and prefix length.
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

    /// Returns a vector of all positions occupied by the word on the grid.
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
