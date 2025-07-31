# Crossword Puzzle Generator

[![GitHub stars](https://img.shields.io/github/stars/mzyui/crossword-puzzle.svg?style=social&label=Star)](https://github.com/mzyui/crossword-puzzle)
[![GitHub forks](https://img.shields.io/github/forks/mzyui/crossword-puzzle.svg?style=social&label=Fork)](https://github.com/mzyui/crossword-puzzle/fork)
[![GitHub issues](https://img.shields.io/github/issues/mzyui/crossword-puzzle.svg)](https://github.com/mzyui/crossword-puzzle/issues)
[![crates.io](https://img.shields.io/crates/v/crossword-puzzle.svg)](https://crates.io/crates/crossword-puzzle)
[![docs.rs](https://docs.rs/crossword-puzzle/badge.svg)](https://docs.rs/crossword-puzzle)

This is a Rust-based command-line application that generates crossword puzzles from a given list of words. It uses a backtracking algorithm to arrange words on a grid, handling placement, resizing, and validation.

## Features

- Generates crossword puzzles from a list of words using a backtracking algorithm.
- Eliminates words that do not share common characters to optimize generation.
- Supports horizontal and vertical word placements.
- Dynamically resizes the grid to accommodate words.
- Includes custom error handling for word and grid operations.

## How to Build

To build the project, navigate to the project's root directory and run the following command:

```bash
cargo build --release
```

This will compile the project and create an executable in the `target/release/` directory.

## How to Run

### Running from Source

After building, you can run the application from the project's root directory:

```bash
./target/release/crossword-puzzle <word1> <word2> ...
```

Replace `<word1> <word2> ...` with the words you want to use in your crossword puzzle. All words must be in uppercase.

### Installation via Cargo

If you have Rust and Cargo installed, you can install the `crossword-puzzle` CLI directly from `crates.io`:

```bash
cargo install crossword-puzzle
```

Once installed, the `crossword-puzzle` executable will be available in your Cargo bin directory (usually `~/.cargo/bin`), allowing you to run it from anywhere in your terminal:

```bash
crossword-puzzle <word1> <word2> ...
```

## Example Usage

To generate a crossword puzzle with the words "LOREM", "IPSUM", "DOLOR", "SIT", and "AMET":

```bash
./target/release/crossword-puzzle LOREM IPSUM DOLOR SIT AMET
```

The application will then print the generated crossword puzzle grid to the console.

```
     A
     M
     E
   SIT
    P
D   S
O   U
LOREM
O
R
```




## JSON Output

The `crossword-puzzle` library and CLI application provide JSON output for the generated grid, which can be useful for programmatic access or integration with other tools. The `Grid` and `Word` structs implement `serde::Serialize`, allowing them to be easily converted to JSON.

```json
{
  "words": [
    {
      "answer": "DOLOR",
      "clue": "",
      "position": {
        "x": 0,
        "y": 5
      },
      "orientation": "vertical"
    },
    {
      "answer": "LOREM",
      "clue": "",
      "position": {
        "x": 0,
        "y": 7
      },
      "orientation": "horizontal"
    },
    {
      "answer": "IPSUM",
      "clue": "",
      "position": {
        "x": 4,
        "y": 3
      },
      "orientation": "vertical"
    },
    {
      "answer": "SIT",
      "clue": "",
      "position": {
        "x": 3,
        "y": 3
      },
      "orientation": "horizontal"
    },
    {
      "answer": "AMET",
      "clue": "",
      "position": {
        "x": 5,
        "y": 0
      },
      "orientation": "vertical"
    }
  ],
  "board": [
    [" "," "," "," "," ","A"],
    [" "," "," "," "," ","M"],
    [" "," "," "," "," ","E"],
    [" "," "," ","S","I","T"],
    [" "," "," "," ","P"," "],
    ["D"," "," "," ","S"," "],
    ["O"," "," "," ","U"," "],
    ["L","O","R","E","M"," "],
    ["O"," "," "," "," "," "],
    ["R"," "," "," "," "," "],
  ]
}
```


### CLI Output

When running the CLI application, the generated grid is printed to the console in a human-readable format, followed by its JSON representation. This allows for both quick visual inspection and easy parsing of the output.

### Library Usage

If you are using `crossword-puzzle` as a library, you can serialize the `Grid` object to JSON using the `to_json()` or `to_json_pretty()` methods:

```rust
use crossword_puzzle::{generate, Grid};

fn main() {
    let words = &["LOREM", "IPSUM", "DOLOR", "SIT", "AMET"];
    match generate(words) {
        Ok(Some(grid)) => {
            // Get compact JSON string
            let json_string = grid.to_json().unwrap();
            println!("Compact JSON: {}", json_string);

            // Get pretty-printed JSON string
            let pretty_json_string = grid.to_json_pretty().unwrap();
            println!("Pretty JSON:\n{}", pretty_json_string);
        },
        _ => (),
    }
}
```

This functionality is enabled by the `serde` feature. Ensure it is enabled in your `Cargo.toml` if you wish to use these methods:

```toml
[dependencies]
crossword-puzzle = {
    version = "*",
    features = ["serde"]
}
```

## Usage as a Library

You can use this crate as a library in your Rust project. Add it to your `Cargo.toml`:

```toml
[dependencies]
crossword-puzzle = "*"
```

Then, you can use the `generate` function to create a crossword puzzle:

```rust
use crossword_puzzle::{generate, Grid};

fn main() {
    let words = &["LOREM", "IPSUM", "DOLOR", "SIT", "AMET"];
    match generate(words) {
        Ok(Some(grid)) => {
            // Print the generated grid
            for row in grid.board {
                println!("{}", row.iter().collect::<String>());
            }
        },
        Ok(None) => println!("Could not generate a crossword puzzle."),
        Err(e) => eprintln!("Error generating puzzle: {}", e),
    }
}
```

## Error Handling

The application includes custom error types for `WordError` and `GridError` to provide informative messages for issues such as:

- Empty or whitespace-only word segments.
- Lowercase characters in word segments (all words must be uppercase).
- Invalid directions for word placement or grid operations.
- Inability to generate a puzzle with the given words.

## To-Do List

- [ ] Implement more sophisticated word placement algorithms.
- [ ] Add support for different grid shapes and sizes.
- [ ] Develop a graphical user interface (GUI).
- [ ] Allow custom word lists from a file.
- [ ] Improve error handling and user feedback.
