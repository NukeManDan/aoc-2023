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

    Ok(input
        .chars()
        .try_fold(0usize, |sum, c| {
            (c == '.').then_some(sum)?;
            c.is_ascii_punctuation().then_some(sum)?;
            Some(sum)
        }).ok_or(AocError::CannotIndex)?
        .to_string())

    // TODO: this line of thinking may be faster... but need to not use iter methods that do a ton of extra .next() calls just to arive from the input at a specific range to compair to.
    //
    // for nrange in range_nums {
    //     if !input
    //         .get(nrange.start - 1..nrange.start)
    //         .ok_or(AocError::CannotIndex)?
    //         .chars()
    //         .fold(true, |acc, c| acc && c != '.' && c.is_ascii_punctuation())
    //     {
    //         continue;
    //     };

    //     if !input
    //         .get(nrange.end + 1..nrange.start)
    //         .ok_or(AocError::CannotIndex)?
    //         .chars()
    //         .fold(true, |acc, c| acc && c != '.' && c.is_ascii_punctuation())
    //     {
    //         continue;
    //     };

    //     if !input
    //         .get((nrange.start + line_len - 1)..(nrange.end + line_len + 1))
    //         .ok_or(AocError::CannotIndex)?
    //         .chars()
    //         .fold(true, |acc, c| acc && c != '.' && c.is_ascii_punctuation())
    //     {
    //         continue;
    //     };

    // }
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
