//! # Day 3: Gear Ratios - Part 1
//!
//! You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.
//!
//! It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
//!
//! "Aaah!"
//!
//! You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.
//!
//! The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.
//!
//! The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
//!
//! Here is an example engine schematic:
//!
//! ```
//! 467..114..
//! ...*......
//! ..35..633.
//! ......#...
//! 617*......
//! .....+.58.
//! ..592.....
//! ......755.
//! ...$.*....
//! .664.598..
//! ```
//!
//! In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
//!
//! Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    // We want to process a line once, collect relevant info and store that, so we don't rerun needlessly.
    // Without heap, this can be done by passing a mutable reference as an argument to your function and use that to copy the value you want to create into the caller's memory space. <https://users.rust-lang.org/t/returning-a-pointer-from-a-function/10110>
    // We can use tail recursion to peek at the next line, see if we keep numbers, and check if symbols exist to keep more in the present line than it indicates on it's own.
    // Although optimization may not happen... <https://stackoverflow.com/questions/59257543/when-is-tail-recursion-guaranteed-in-rust>
    // A mutable pointer to an enumerated iter with elements

    // ASSUME: All lines are same length!
    let line_len = input.find('\n').ok_or(AocError::BadInput)?;

    // Collect the byte index of each punctuation, this will be in incremental order, unless latter done on threads...
    let mut punct_pos = Vec::with_capacity(input.len() / 4);
    // Collect the index range for every number, and the value within
    let mut digit_indicies = Vec::with_capacity(input.len() / 4);

    let mut maybe_buf: Option<(Option<usize>, String)> = None;
    for (idx, c) in input.chars().enumerate() {
        if maybe_buf.is_none() {
            if c == '.' {
                continue;
            }
            if c.is_ascii_punctuation() {
                punct_pos.push(idx);
                continue;
            }
        }
        if let Some(buf) = maybe_buf.as_mut() {
            if c.is_ascii_digit() {
                buf.1.push(c);
                if buf.0.is_none() {
                    buf.0 = Some(idx)
                };
            } else {
                if c.is_ascii_punctuation() {
                    punct_pos.push(idx);
                }
                // we are one past the number, ala Range (start..end
                // std::ops::Range {start: buf.0.unwrap(), end: idx}
                digit_indicies.push(buf.1.parse::<usize>());
                maybe_buf = None;
            }
        }
    }

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("114", process(input)?);
        Ok(())
    }
}
