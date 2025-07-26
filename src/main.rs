use crossword_puzzle::generate;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <word1> <word2> ...", args[0]);
        return;
    }

    let words: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();

    match generate(&words) {
        Ok(Some(grid)) => {
            println!("Generated Crossword Puzzle:");
            for row in grid.board {
                println!("{}", row.iter().collect::<String>());
            }
        }
        Ok(None) => {
            println!("Could not generate a crossword puzzle with the given words.");
        }
        Err(e) => {
            eprintln!("Error generating puzzle: {e}");
        }
    }
}
