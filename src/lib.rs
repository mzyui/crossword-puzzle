use std::collections::VecDeque;
use std::fmt::Debug;

use crate::error::{Error, GridError, WordError};
use crate::word::{Direction, Position, Word};

pub mod error;
pub mod word;

/// `Neighbor` represents the characters and their positions in the cells immediately adjacent to a given position on the crossword grid.
/// It is used internally to check for conflicts or valid placements when adding words.
#[derive(Debug, Default)]
pub struct Neighbor {
    /// The character and `Position` of the cell directly above the current position.
    /// `None` if the cell is out of bounds or empty.
    pub up: Option<(Position, char)>,
    /// The character and `Position` of the cell directly to the right of the current position.
    /// `None` if the cell is out of bounds or empty.
    pub right: Option<(Position, char)>,
    /// The character and `Position` of the cell directly below the current position.
    /// `None` if the cell is out of bounds or empty.
    pub down: Option<(Position, char)>,
    /// The character and `Position` of the cell directly to the left of the current position.
    /// `None` if the cell is out of bounds or empty.
    pub left: Option<(Position, char)>,
}

/// `Grid` represents the crossword puzzle board and manages the placement and validation of words.
/// It dynamically resizes to accommodate words and provides methods for adding words and finding valid placements.
#[derive(Clone, Debug)]
pub struct Grid<'a> {
    /// A collection of `Word`s that have been successfully placed on the grid.
    pub words: Vec<Word<'a>>,
    /// The 2D vector of characters representing the crossword board itself.
    /// Empty cells are typically represented by a space character (' ').
    pub board: Vec<Vec<char>>,
}

impl<'a> Default for Grid<'a> {
    /// Creates a new empty `Grid`.
    fn default() -> Self {
        Self::new()
    }
}

type GetPosFn<'a> = Box<dyn Fn(&Word<'a>, usize) -> (usize, usize)>;
type GetOriginFn<'a> = Box<dyn Fn(&Word<'a>) -> usize>;
type PlacementHelper<'a> = (Direction, GetPosFn<'a>, GetOriginFn<'a>);

impl<'a> Grid<'a> {
    /// Creates a new, empty `Grid` instance.
    /// The grid is initialized with a single empty cell (`' '`).
    ///
    /// # Returns
    ///
    /// A new `Grid` with an empty `words` vector and a `board` containing one empty cell.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::Grid;
    ///
    /// let grid = Grid::new();
    /// assert!(grid.words.is_empty());
    /// assert_eq!(grid.board, vec![vec![' ']]);
    /// ```
    pub fn new() -> Self {
        Self {
            words: vec![],
            board: vec![vec![' ']],
        }
    }

    /// Adds a `Word` to the grid.
    ///
    /// This function first ensures the grid is large enough to accommodate the new word,
    /// then updates the word's internal position, fills the corresponding cells on the board
    /// with the word's characters, and finally adds the word to the grid's list of placed words.
    ///
    /// # Arguments
    ///
    /// * `word` - A mutable `Word` instance to be added to the grid.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the word was successfully added.
    /// - `Err(GridError)` if there was an issue with resizing the grid or filling the word.
    ///
    /// # Errors
    ///
    /// Returns a `GridError` if:
    /// - The grid cannot be resized to fit the word.
    /// - The word's direction is `NotSet` during filling.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::{Grid, word::{Word, Direction}};
    ///
    /// let mut grid = Grid::new();
    /// let word = Word::value("", 'R', "UST").unwrap().direction(Direction::Horizontal);
    /// assert!(grid.add_word(word).is_ok());
    /// ```
    pub fn add_word(&mut self, mut word: Word<'a>) -> Result<(), GridError> {
        self.ensure_grid_size(&mut word)?;
        word.update_position();
        self.fill_word(&word)?;

        self.words.push(word);

        for word in self.words.iter_mut() {
            word.update_position();
        }

        Ok(())
    }

    /// Recursively resizes the grid by adding empty cells in the specified direction.
    ///
    /// This function expands the grid by `amount` in the given `direction`.
    /// If `is_prepend` is `true`, cells are added at the beginning (top or left);
    /// otherwise, they are added at the end (bottom or right).
    /// Word positions are adjusted accordingly if cells are prepended.
    ///
    /// # Arguments
    ///
    /// * `amount` - The number of cells to add.
    /// * `direction` - The `Direction` in which to resize the grid (Horizontal or Vertical).
    /// * `is_prepend` - A boolean indicating whether to add cells at the beginning (`true`) or end (`false`).
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the grid was successfully resized.
    /// - `Err(GridError)` if an invalid direction is provided.
    ///
    /// # Errors
    ///
    /// Returns a `GridError::InvalidDirection` if `Direction::NotSet` is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::{Grid, word::Direction};
    ///
    /// let mut grid = Grid::new();
    /// // Resize horizontally by 5 cells, prepending them
    /// assert!(grid.resize_grid(5, Direction::Horizontal, true).is_ok());
    /// // Resize vertically by 3 cells, appending them
    /// assert!(grid.resize_grid(3, Direction::Vertical, false).is_ok());
    /// ```
    pub fn resize_grid(
        &mut self,
        amount: usize,
        direction: Direction,
        is_prepend: bool,
    ) -> Result<(), GridError> {
        if amount == 0 {
            return Ok(());
        }

        match direction {
            Direction::Horizontal => {
                for row in self.board.iter_mut() {
                    if is_prepend {
                        row.insert(0, ' ');
                    } else {
                        row.push(' ');
                    }
                }

                if is_prepend {
                    for word in self.words.iter_mut() {
                        word.position.x += 1;
                    }
                }
            }
            Direction::Vertical => {
                let length = self.board[0].len();
                if is_prepend {
                    self.board.insert(0, [' '].repeat(length));
                } else {
                    self.board.push([' '].repeat(length));
                }
                if is_prepend {
                    for word in self.words.iter_mut() {
                        word.position.y += 1;
                    }
                }
            }
            Direction::NotSet => {
                return Err(GridError::InvalidDirection(
                    "Invalid direction for grid resize.".to_string(),
                ))
            }
        }

        self.resize_grid(amount - 1, direction, is_prepend)
    }

    /// Ensures the grid is large enough to accommodate a given `Word`.
    ///
    /// This function checks if the word, including its prefix and suffix, would extend
    /// beyond the current grid boundaries. If necessary, it resizes the grid by calling
    /// `resize_grid` and adjusts the word's position to reflect the new grid dimensions.
    ///
    /// # Arguments
    ///
    /// * `word` - A mutable reference to the `Word` to be placed.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the grid is successfully ensured to be large enough.
    /// - `Err(GridError)` if there's an issue during grid resizing.
    ///
    /// # Errors
    ///
    /// Returns a `GridError` if `resize_grid` encounters an error (e.g., `InvalidDirection`).
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::{Grid, word::{Word, Direction}};
    ///
    /// let mut grid = Grid::new();
    /// let mut word = Word::value("", 'A', "BC").unwrap().position(0, 0).direction(Direction::Horizontal);
    /// // Initially, the grid is 1x1. This will cause it to resize.
    /// assert!(grid.ensure_grid_size(&mut word).is_ok());
    /// ```
    pub fn ensure_grid_size(&mut self, word: &mut Word<'a>) -> Result<(), GridError> {
        let position = &mut word.position;
        let segment = &word.segment;

        let prefix_pos = if word.direction == Direction::Horizontal {
            position.x as isize - segment.prefix.len() as isize
        } else {
            position.y as isize - segment.prefix.len() as isize
        };

        if prefix_pos < 0 {
            let abs_prefix_pos = prefix_pos.unsigned_abs();
            self.resize_grid(abs_prefix_pos, word.direction, true)?;

            if word.direction == Direction::Horizontal {
                position.x += abs_prefix_pos;
            } else {
                position.y += abs_prefix_pos;
            }
        }

        let suffix_pos = if word.direction == Direction::Horizontal {
            let length = position.x.saturating_add(segment.suffix.len()) + 1;
            self.board[0].len() as isize - length as isize
        } else {
            let length = position.y.saturating_add(segment.suffix.len()) + 1;
            self.board.len() as isize - length as isize
        };
        if suffix_pos < 0 {
            self.resize_grid(suffix_pos.unsigned_abs(), word.direction, false)?;
        }

        Ok(())
    }

    /// Fills the grid with the characters of a given `Word`.
    ///
    /// This function iterates through the characters of the word and places them onto the `board`
    /// at the word's calculated `position` and `direction`.
    ///
    /// # Arguments
    ///
    /// * `word` - A reference to the `Word` whose characters will fill the grid.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the word was successfully filled onto the grid.
    /// - `Err(GridError)` if an invalid direction is provided.
    ///
    /// # Errors
    ///
    /// Returns a `GridError::InvalidDirection` if `Direction::NotSet` is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::{Grid, word::{Word, Direction}};
    ///
    /// let mut grid = Grid::new();
    /// let word = Word::value("", 'T', "EST").unwrap().position(0, 0).direction(Direction::Horizontal);
    /// // Assuming grid is large enough and word position is valid
    /// assert!(grid.fill_word(&word).is_ok());
    /// ```
    pub fn fill_word(&mut self, word: &Word<'a>) -> Result<(), GridError> {
        match word.direction {
            Direction::Horizontal => {
                for (ch, index) in word.segment.full_word().iter().zip(word.origin.x..) {
                    self.board[word.position.y][index] = *ch;
                }
            }
            Direction::Vertical => {
                for (ch, index) in word.segment.full_word().iter().zip(word.origin.y..) {
                    self.board[index][word.position.x] = *ch;
                }
            }
            Direction::NotSet => {
                return Err(GridError::InvalidDirection(
                    "Invalid direction for filling word.".to_string(),
                ))
            }
        }
        Ok(())
    }

    /*
     * VALIDATOR
     */

    /// Returns the character at the given `Position` on the grid, if it exists.
    ///
    /// This function safely retrieves a character from the `board` at the specified `Position`,
    /// returning `None` if the position is out of bounds.
    ///
    /// # Arguments
    ///
    /// * `position` - The `Position` (x, y) on the grid to retrieve the character from.
    ///
    /// # Returns
    ///
    /// - `Some(char)` if a character exists at the given position.
    /// - `None` if the position is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::{Grid, word::{Word, Direction, Position}};
    ///
    /// let mut grid = Grid::new();
    /// let word = Word::value("", 'A', "").unwrap().position(0, 0).direction(Direction::Horizontal);
    /// grid.add_word(word).unwrap();
    ///
    /// assert_eq!(grid.get_char(Position { x: 0, y: 0 }), Some('A'));
    /// assert_eq!(grid.get_char(Position { x: 1, y: 0 }), None);
    /// ```
    pub fn get_char(&self, position: Position) -> Option<char> {
        self.board
            .get(position.y)
            .and_then(|col| col.get(position.x))
            .copied()
    }

    /// Helper function to get a neighbor at a given offset from the current position.
    ///
    /// Returns `Some((Position, char))` if the neighbor exists and is within bounds,
    /// otherwise `None`.
    ///
    /// # Arguments
    ///
    /// * `current_pos` - The starting `Position`.
    /// * `dx` - The x-offset (change in column).
    /// * `dy` - The y-offset (change in row).
    ///
    /// # Returns
    ///
    /// - `Some((Position, char))` if the neighbor exists and is within bounds.
    /// - `None` if the new position is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::{Grid, word::{Word, Direction, Position}};
    ///
    /// let mut grid = Grid::new();
    /// let word = Word::value("", 'A', "").unwrap().position(0, 0).direction(Direction::Horizontal);
    /// grid.add_word(word).unwrap();
    ///
    /// // Get the character at (0,0) (the current position)
    /// assert_eq!(grid.get_neighbor_at_offset(Position { x: 0, y: 0 }, 0, 0), Some((Position { x: 0, y: 0 }, 'A')));
    /// // Try to get a neighbor out of bounds
    /// assert_eq!(grid.get_neighbor_at_offset(Position { x: 0, y: 0 }, -1, 0), None);
    /// ```
    pub fn get_neighbor_at_offset(
        &self,
        current_pos: Position,
        dx: isize,
        dy: isize,
    ) -> Option<(Position, char)> {
        let new_x = current_pos.x as isize + dx;
        let new_y = current_pos.y as isize + dy;

        if new_x >= 0 && new_y >= 0 {
            let new_pos = Position {
                x: new_x as usize,
                y: new_y as usize,
            };
            self.get_char(new_pos).map(|ch| (new_pos, ch))
        } else {
            None
        }
    }

    /// Returns the `Neighbor` struct for a given `Position` on the grid.
    ///
    /// This function checks the cells immediately above, right, below, and left of the given `position`.
    /// It returns a `Neighbor` struct containing the character and position of each existing neighbor.
    ///
    /// # Arguments
    ///
    /// * `position` - The `Position` for which to find neighbors.
    ///
    /// # Returns
    ///
    /// A `Neighbor` struct populated with the characters and positions of adjacent cells.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::{Grid, word::{Word, Direction, Position}, Neighbor};
    ///
    /// let mut grid = Grid::new();
    /// let word = Word::value("", 'A', "").unwrap().position(0, 0).direction(Direction::Horizontal);
    /// grid.add_word(word).unwrap();
    ///
    /// let neighbors = grid.get_neighbor(Position { x: 0, y: 0 });
    /// assert!(neighbors.up.is_none());
    /// assert!(neighbors.right.is_none());
    /// assert!(neighbors.down.is_none());
    /// assert!(neighbors.left.is_none());
    /// ```
    pub fn get_neighbor(&self, position: Position) -> Neighbor {
        Neighbor {
            up: self.get_neighbor_at_offset(position, 0, -1),
            right: self.get_neighbor_at_offset(position, 1, 0),
            down: self.get_neighbor_at_offset(position, 0, 1),
            left: self.get_neighbor_at_offset(position, -1, 0),
        }
    }

    /// Calculates the next `Position` based on the current position, direction, and step.
    ///
    /// This helper function determines the new `Position` by moving `step` units from `current_pos`
    /// in the specified `direction` (Horizontal or Vertical).
    ///
    /// # Arguments
    ///
    /// * `current_pos` - The starting `Position`.
    /// * `direction` - The `Direction` of movement (Horizontal or Vertical).
    /// * `step` - The number of units to move (can be negative for backward movement).
    ///
    /// # Returns
    ///
    /// - `Ok(Position)` representing the new calculated position.
    /// - `Err(GridError)` if an invalid direction is provided.
    ///
    /// # Errors
    ///
    /// Returns a `GridError::InvalidDirection` if `Direction::NotSet` is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::{Grid, word::{Direction, Position}};
    ///
    /// let grid = Grid::new();
    /// let pos = Position { x: 5, y: 5 };
    ///
    /// // Move horizontally by 2
    /// assert_eq!(grid.get_next_pos(pos, Direction::Horizontal, 2).unwrap(), Position { x: 7, y: 5 });
    /// // Move vertically by -3
    /// assert_eq!(grid.get_next_pos(pos, Direction::Vertical, -3).unwrap(), Position { x: 5, y: 2 });
    /// ```
    pub fn get_next_pos(
        &self,
        current_pos: Position,
        direction: Direction,
        step: isize,
    ) -> Result<Position, GridError> {
        match direction {
            Direction::Horizontal => Ok(Position {
                x: (current_pos.x as isize + step) as usize,
                y: current_pos.y,
            }),
            Direction::Vertical => Ok(Position {
                x: current_pos.x,
                y: (current_pos.y as isize + step) as usize,
            }),
            Direction::NotSet => Err(GridError::InvalidDirection("Invalid direction".to_string())),
        }
    }

    /// Retrieves the coordinate value (x or y) based on the given direction.
    ///
    /// This helper function extracts either the `x` or `y` coordinate from `current_pos`
    /// depending on the provided `direction` (Horizontal or Vertical).
    ///
    /// # Arguments
    ///
    /// * `current_pos` - The `Position` from which to extract the coordinate.
    /// * `direction` - The `Direction` indicating which coordinate to retrieve (Horizontal for x, Vertical for y).
    ///
    /// # Returns
    ///
    /// - `Ok(usize)` representing the extracted coordinate value.
    /// - `Err(GridError)` if an invalid direction is provided.
    ///
    /// # Errors
    ///
    /// Returns a `GridError::InvalidDirection` if `Direction::NotSet` is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::{Grid, word::{Direction, Position}};
    ///
    /// let grid = Grid::new();
    /// let pos = Position { x: 10, y: 20 };
    ///
    /// assert_eq!(grid.get_coord_val(pos, Direction::Horizontal).unwrap(), 10);
    /// assert_eq!(grid.get_coord_val(pos, Direction::Vertical).unwrap(), 20);
    /// ```
    pub fn get_coord_val(
        &self,
        current_pos: Position,
        direction: Direction,
    ) -> Result<usize, GridError> {
        match direction {
            Direction::Horizontal => Ok(current_pos.x),
            Direction::Vertical => Ok(current_pos.y),
            Direction::NotSet => Err(GridError::InvalidDirection("Invalid direction".to_string())),
        }
    }

    /// Helper function to check if a character option is empty or contains a space.
    ///
    /// This function is used to determine if a cell on the grid is effectively empty.
    ///
    /// # Arguments
    ///
    /// * `char_option` - An `Option` containing a `(Position, char)` tuple.
    ///
    /// # Returns
    ///
    /// - `true` if `char_option` is `None` or if it contains a space character (`' '`).
    /// - `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::{Grid, word::Position};
    ///
    /// let grid = Grid::new();
    ///
    /// assert!(grid.is_char_empty_or_none(None));
    /// assert!(grid.is_char_empty_or_none(Some((Position { x: 0, y: 0 }, ' '))));
    /// assert!(!grid.is_char_empty_or_none(Some((Position { x: 0, y: 0 }, 'A'))));
    /// ```
    pub fn is_char_empty_or_none(&self, char_option: Option<(Position, char)>) -> bool {
        match char_option {
            Some((_, ch)) => ch == ' ',
            None => true,
        }
    }

    /// Checks if the neighboring cells in the specified direction are empty.
    ///
    /// This function is used to validate if a word can be placed without conflicting with
    /// existing characters in adjacent cells (not directly part of the word's path).
    /// It examines the cells perpendicular to the word's direction at `current_pos`.
    /// For `Direction::Horizontal`, it checks `up` and `down` neighbors. For `Direction::Vertical`,
    /// it checks `left` and `right` neighbors.
    ///
    /// # Arguments
    ///
    /// * `current_pos` - The `Position` to check neighbors around.
    /// * `direction` - The `Direction` of the word being placed (Horizontal or Vertical).
    ///
    /// # Returns
    ///
    /// - `Ok(true)` if the relevant neighboring cells are empty.
    /// - `Ok(false)` if any relevant neighboring cell contains a character.
    /// - `Err(GridError)` if an invalid direction is provided.
    ///
    /// # Errors
    ///
    /// Returns a `GridError::InvalidDirection` if `Direction::NotSet` is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::{Grid, word::{Word, Direction, Position}};
    ///
    /// let mut grid = Grid::new();
    /// // Initially, all cells are empty
    /// assert!(grid.is_neighbor_cell_empty(Position { x: 0, y: 0 }, Direction::Horizontal).unwrap());
    ///
    /// let word = Word::value("", 'A', "").unwrap().position(0, 1).direction(Direction::Horizontal);
    /// grid.add_word(word).unwrap(); // Place 'A' at (0,1)
    ///
    /// // Now, checking (0,0) horizontally should return false because (0,1) is occupied
    /// assert!(!grid.is_neighbor_cell_empty(Position { x: 0, y: 0 }, Direction::Horizontal).unwrap());
    /// ```
    pub fn is_neighbor_cell_empty(
        &self,
        current_pos: Position,
        direction: Direction,
    ) -> Result<bool, GridError> {
        let neighbor = self.get_neighbor(current_pos);

        match direction {
            Direction::Horizontal => Ok(self.is_char_empty_or_none(neighbor.up)
                && self.is_char_empty_or_none(neighbor.down)),
            Direction::Vertical => Ok(self.is_char_empty_or_none(neighbor.left)
                && self.is_char_empty_or_none(neighbor.right)),
            Direction::NotSet => Err(GridError::InvalidDirection("Invalid direction".to_string())),
        }
    }

    /// Checks if a `Word` can be validly placed on the grid.
    ///
    /// This function performs a series of checks to ensure that placing the given `word`
    /// on the grid at its specified position and direction does not violate any crossword rules.
    /// It verifies that the word does not overlap with existing characters incorrectly and that
    /// adjacent cells are empty where required.
    ///
    /// # Arguments
    ///
    /// * `word` - A reference to the `Word` to validate for placement.
    ///
    /// # Returns
    ///
    /// - `Ok(true)` if the word can be validly placed.
    /// - `Ok(false)` if the placement is invalid due to conflicts or rule violations.
    /// - `Err(GridError)` if an invalid direction is provided during internal checks.
    ///
    /// # Errors
    ///
    /// Returns a `GridError` if any internal helper function (e.g., `get_next_pos`,
    /// `is_neighbor_cell_empty`) encounters an `InvalidDirection` error.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::{Grid, word::{Word, Direction}};
    ///
    /// let mut grid = Grid::new();
    /// let word1 = Word::value("", 'A', "").unwrap().position(0, 0).direction(Direction::Horizontal);
    /// grid.add_word(word1).unwrap();
    ///
    /// // A word that overlaps correctly
    /// let word2 = Word::value("", 'A', "").unwrap().position(0, 0).direction(Direction::Vertical);
    /// assert!(grid.is_valid_placement(&word2).unwrap());
    ///
    /// // A word that conflicts
    /// let word3 = Word::value("", 'B', "").unwrap().position(0, 0).direction(Direction::Horizontal);
    /// assert!(!grid.is_valid_placement(&word3).unwrap());
    /// ```
    pub fn is_valid_placement(&self, word: &Word<'a>) -> Result<bool, GridError> {
        let mut pos = word.position;

        // Check prefix
        if !self.check_segment_placement(word, &mut pos, word.segment.prefix.chars().rev(), -1)? {
            return Ok(false);
        }

        // Check suffix
        pos = word.position;
        if !self.check_segment_placement(word, &mut pos, word.segment.suffix.chars(), 1)? {
            return Ok(false);
        }

        Ok(true)
    }

    /// Helper function to check the placement of a word segment (prefix or suffix).
    ///
    /// This function iterates through the characters of a word segment and validates their
    /// placement on the grid. It checks for conflicts with existing characters and ensures
    /// that perpendicular neighbors are empty where required.
    ///
    /// # Arguments
    ///
    /// * `word` - A reference to the `Word` being checked.
    /// * `pos` - A mutable reference to the current `Position` on the grid, which is updated during iteration.
    /// * `chars` - An iterator over the characters of the segment (prefix or suffix).
    /// * `step` - The step size and direction (e.g., `1` for forward, `-1` for backward).
    ///
    /// # Returns
    ///
    /// - `Ok(true)` if the segment can be validly placed.
    /// - `Ok(false)` if a conflict is found.
    /// - `Err(GridError)` if an invalid direction is provided during internal checks.
    ///
    /// # Errors
    ///
    /// Returns a `GridError` if any internal helper function (e.g., `get_next_pos`,
    /// `is_neighbor_cell_empty`) encounters an `InvalidDirection` error.
    pub fn check_segment_placement(
        &self,
        word: &Word<'a>,
        pos: &mut Position,
        chars: impl Iterator<Item = char>,
        step: isize,
    ) -> Result<bool, GridError> {
        for ch in chars {
            *pos = self.get_next_pos(*pos, word.direction, step)?;
            if let Some(board_ch) = self.get_char(*pos) {
                if board_ch == ch {
                    continue;
                }
                if board_ch != ' ' && board_ch != ch {
                    return Ok(false);
                }
                if !self.is_neighbor_cell_empty(*pos, word.direction)? {
                    return Ok(false);
                }
                if self.get_coord_val(*pos, word.direction)? != 0 {
                    continue;
                }
            }
            break;
        }
        // After the loop, check the character at the next position if the iterator is exhausted
        *pos = self.get_next_pos(*pos, word.direction, step)?;
        if let Some(board_ch) = self.get_char(*pos) {
            if board_ch != ' ' {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Finds valid placements for a word segment based on a crossed character.
    ///
    /// This function searches for existing words on the grid that can be crossed by a new word segment.
    /// It generates potential `Word` candidates based on the `prefix`, `crossed` character, `suffix`,
    /// and `direction`, and then validates each potential placement using `is_valid_placement`.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The string segment before the crossed character.
    /// * `crossed` - The character that crosses an existing word.
    /// * `suffix` - The string segment after the crossed character.
    /// * `direction` - The `Direction` of the new word (Horizontal or Vertical).
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Word>)` containing a list of valid `Word` placements.
    /// - `Err(GridError)` if an invalid direction is provided or a `WordError` occurs during `Word::value` creation.
    ///
    /// # Errors
    ///
    /// Returns a `GridError` if `Direction::NotSet` is provided or if `Word::value` returns a `WordError`.
    pub fn find_valid_placements_for_segment(
        &self,
        prefix: &'a str,
        crossed: char,
        suffix: &'a str,
        direction: Direction,
    ) -> Result<Vec<Word<'a>>, GridError> {
        let mut placements = Vec::new();
        let (opposite_direction, get_pos, get_origin): PlacementHelper<'a> = match direction {
            Direction::Horizontal => (
                Direction::Vertical,
                Box::new(|w, i| (w.position.x, i)),
                Box::new(|w| w.origin.y),
            ),
            Direction::Vertical => (
                Direction::Horizontal,
                Box::new(|w, i| (i, w.position.y)),
                Box::new(|w| w.origin.x),
            ),
            Direction::NotSet => {
                return Err(GridError::InvalidDirection("Invalid direction".to_string()))
            }
        };

        for word in self
            .words
            .iter()
            .filter(|p| p.direction == opposite_direction)
        {
            let full_word = word.segment.full_word();
            for (ch, index) in full_word.iter().zip(get_origin(word)..) {
                if *ch == crossed {
                    let (x, y) = get_pos(word, index);
                    let new_word = Word::value(prefix, crossed, suffix)?
                        .position(x, y)
                        .direction(direction);

                    if self.is_valid_placement(&new_word)? {
                        placements.push(new_word);
                    }
                }
            }
        }

        Ok(placements)
    }

    /// Finds all valid placements for a given word string on the current grid.
    ///
    /// This function iterates through each character of the `word_str` to consider it as a potential
    /// crossing point. For each potential crossing, it attempts to find valid horizontal and vertical
    /// placements on the grid. If the grid is empty, it considers initial horizontal and vertical placements.
    ///
    /// # Arguments
    ///
    /// * `word_str` - The string slice representing the word to find placements for.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Word>)` containing a list of all valid `Word` placements found.
    /// - `Err(GridError)` if an invalid direction is provided or a `WordError` occurs during `Word` creation.
    ///
    /// # Errors
    ///
    /// Returns a `GridError` if `handle_initial_placements` or `find_valid_placements_for_segment`
    /// return an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::{Grid, word::{Word, Direction}};
    ///
    /// let mut grid = Grid::new();
    /// let word1 = Word::value("", 'R', "UST").unwrap().direction(Direction::Horizontal);
    /// grid.add_word(word1).unwrap();
    ///
    /// let placements = grid.find_valid_placements("TEST").unwrap();
    /// assert!(!placements.is_empty());
    /// ```
    pub fn find_valid_placements(&self, word_str: &'a str) -> Result<Vec<Word<'a>>, GridError> {
        let mut placements = Vec::new();

        for index in 0..word_str.len() {
            let (prefix, remain) = word_str.split_at(index);
            let (mid, suffix) = remain.split_at(1);
            let crossed = mid.chars().next().unwrap();

            if self.words.is_empty() {
                placements.extend(self.handle_initial_placements(prefix, crossed, suffix)?);
            } else {
                placements.extend(self.find_valid_placements_for_segment(
                    prefix,
                    crossed,
                    suffix,
                    Direction::Horizontal,
                )?);
                placements.extend(self.find_valid_placements_for_segment(
                    prefix,
                    crossed,
                    suffix,
                    Direction::Vertical,
                )?);
            }
        }

        Ok(placements)
    }

    /// Handles the initial placements when the grid is empty.
    ///
    /// It generates both horizontal and vertical `Word` placements for the given segment,
    /// assuming an empty grid where the word can be placed at the origin (0,0).
    ///
    /// # Arguments
    ///
    /// * `prefix` - The string segment before the crossed character.
    /// * `crossed` - The character that would be at the crossing point.
    /// * `suffix` - The string segment after the crossed character.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Word>)` containing two `Word` candidates: one horizontal and one vertical.
    /// - `Err(GridError)` if a `WordError` occurs during `Word::value` creation.
    ///
    /// # Errors
    ///
    /// Returns a `GridError` if `Word::value` returns a `WordError` (e.g., invalid segment).
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::{Grid, word::Direction};
    ///
    /// let grid = Grid::new();
    /// let placements = grid.handle_initial_placements("", 'A', "BC").unwrap();
    /// assert_eq!(placements.len(), 2);
    /// assert_eq!(placements[0].direction, Direction::Horizontal);
    /// assert_eq!(placements[1].direction, Direction::Vertical);
    /// ```
    pub fn handle_initial_placements(
        &self,
        prefix: &'a str,
        crossed: char,
        suffix: &'a str,
    ) -> Result<Vec<Word<'a>>, GridError> {
        let mut placements = Vec::new();
        let horizontal_word =
            Word::value(prefix, crossed, suffix)?.direction(Direction::Horizontal);
        placements.push(horizontal_word);

        let vertical_word = Word::value(prefix, crossed, suffix)?.direction(Direction::Vertical);
        placements.push(vertical_word);
        Ok(placements)
    }
}

#[derive(Clone, Debug)]
pub struct PossibleWord<'a> {
    pub value: &'a str,
    pub remaining: usize,
}

impl<'a> PossibleWord<'a> {
    /// Creates a new `PossibleWord` instance.
    ///
    /// Initializes a `PossibleWord` with the given string `value` and sets
    /// `remaining` attempts to `3` by default.
    ///
    /// # Arguments
    ///
    /// * `value` - The string slice representing the word.
    ///
    /// # Returns
    ///
    /// A new `PossibleWord` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossword_puzzle::PossibleWord;
    ///
    /// let pw = PossibleWord::new("HELLO");
    /// assert_eq!(pw.value, "HELLO");
    /// assert_eq!(pw.remaining, 3);
    /// ```
    pub fn new(value: &'a str) -> Self {
        Self {
            value,
            remaining: 3,
        }
    }
}

/// A backtracking function to generate the crossword puzzle.
///
/// This function attempts to place words one by one onto the grid using a recursive
/// backtracking approach. It explores possible placements for each word and, if a
/// placement leads to a dead end, it backtracks to try another path.
///
/// # Arguments
///
/// * `grid` - The current `Grid` state.
/// * `words_to_place` - A `VecDeque` containing `PossibleWord`s that still need to be placed.
///
/// # Returns
///
/// - `Ok(Some(Grid))` if a complete and valid crossword puzzle grid is successfully generated.
/// - `Ok(None)` if no valid grid can be generated from the given words.
/// - `Err(Error)` if an error occurs during grid operations (e.g., invalid word segments).
///
/// # Errors
///
/// Returns an `Error` if `Grid::find_valid_placements` or `Grid::add_word` return an error.
pub fn backtrack<'a>(
    grid: Grid<'a>,
    mut words_to_place: VecDeque<PossibleWord<'a>>,
) -> Result<Option<Grid<'a>>, Error> {
    if let Some(mut current_word) = words_to_place.pop_front() {
        let placements = grid.find_valid_placements(current_word.value)?;
        if placements.is_empty() && current_word.remaining > 1 {
            current_word.remaining = current_word.remaining.saturating_sub(1);
            words_to_place.push_back(current_word);
            return backtrack(grid, words_to_place);
        }
        for placement_word in placements {
            let mut new_grid = grid.clone();
            new_grid.add_word(placement_word)?;

            if let Some(final_grid) = backtrack(new_grid, words_to_place.clone())? {
                return Ok(Some(final_grid));
            }
        }
    }

    Ok((!grid.words.is_empty() || words_to_place.is_empty()).then_some(grid))
}

/// Eliminates words that do not share any common characters with other words.
///
/// This function filters the initial list of words, keeping only those that have at least
/// one common character with another word in the list. This helps in reducing the search space
/// for the crossword generation by focusing on words that can actually intersect.
/// The words are then sorted by length in reverse order (longest first).
///
/// # Arguments
///
/// * `words_to_place` - A slice of string slices (`&[&'a str]`) representing the initial list of words.
///
/// # Returns
///
/// A `VecDeque<PossibleWord>` containing the filtered and sorted words, wrapped in `PossibleWord` structs.
///
/// # Examples
///
/// ```
/// use crossword_puzzle::{eliminate_words, PossibleWord};
/// use std::collections::VecDeque;
///
/// let words = &["RUST", "TEST", "CODE", "APPLE"];
/// let filtered_words = eliminate_words(words);
///
/// // "APPLE" does not share any common characters with "RUST", "TEST", or "CODE"
/// // So it should be eliminated.
/// assert_eq!(filtered_words.len(), 3);
/// assert_eq!(filtered_words.front().unwrap().value, "RUST");
/// ```
pub fn eliminate_words<'a>(words_to_place: &[&'a str]) -> VecDeque<PossibleWord<'a>> {
    let mut possible_words = Vec::new();

    for word_str in words_to_place.iter() {
        for word_str_cmp in words_to_place.iter() {
            if word_str == word_str_cmp {
                continue;
            }

            let mut chars = word_str_cmp.chars().collect::<Vec<_>>();
            chars.dedup();

            if word_str.chars().any(|ch| chars.contains(&ch)) {
                if !possible_words.contains(word_str) {
                    possible_words.push(*word_str);
                }
                if !possible_words.contains(word_str_cmp) {
                    possible_words.push(word_str_cmp);
                }
                break;
            }
        }
    }
    possible_words.sort_by_key(|c| std::cmp::Reverse(c.len()));
    VecDeque::from(
        possible_words
            .iter()
            .map(|w| PossibleWord::new(w))
            .collect::<Vec<_>>(),
    )
}

/// Generates a crossword puzzle grid from a given list of words.
///
/// This function takes a slice of string slices, where each string represents a word
/// to be included in the crossword puzzle. It attempts to arrange these words
/// into a valid crossword grid using a backtracking algorithm.
///
/// The process involves:
/// 1. Eliminating words that do not share common characters with other words to
///    reduce the search space.
/// 2. Attempting to place words one by one onto the grid.
/// 3. If a word cannot be placed, the algorithm backtracks and tries a different
///    placement or a different word.
///
/// # Arguments
///
/// * `words` - A slice of string slices (`&[&'a str]`) representing the words
///   to be used in the crossword puzzle.
///
/// # Returns
///
/// A `Result` which is:
/// - `Ok(Some(Grid))` if a valid crossword puzzle grid is successfully generated.
/// - `Ok(None)` if no valid grid can be generated from the given words.
/// - `Err(Error)` if an error occurs during the generation process (e.g.,
///   invalid word segments).
///
/// # Examples
///
/// ```
/// use crossword_puzzle::{generate, Grid};
///
/// let words = &["LOREM", "IPSUM", "DOLOR", "SIT", "AMET"];
/// match generate(words) {
///     Ok(Some(grid)) => {
///         // Print the generated grid
///         for row in grid.board {
///             println!("{}", row.iter().collect::<String>());
///         }
///     },
///     Ok(None) => println!("Could not generate a crossword puzzle."),
///     Err(e) => eprintln!("Error generating puzzle: {}", e),
/// }
/// ```
pub fn generate<'a>(words: &[&'a str]) -> Result<Option<Grid<'a>>, Error> {
    for word in words.iter() {
        if word.chars().any(|c| c.is_lowercase()) {
            return Err(Error::WordError(WordError::LowercaseCharactersInSegment));
        }
    }

    let words_queue = eliminate_words(words);
    let initial_grid = Grid::new();
    backtrack(initial_grid, words_queue)
}
