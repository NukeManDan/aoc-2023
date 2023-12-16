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

use lazy_regex::regex;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    // We want to process a line once, collect relevant info and store that, so we don't rerun needlessly.
    // Without heap, this can be done by passing a mutable reference as an argument to your function and use that to copy the value you want to create into the caller's memory space. <https://users.rust-lang.org/t/returning-a-pointer-from-a-function/10110>
    // We can use tail recursion to peek at the next line, see if we keep numbers, and check if symbols exist to keep more in the present line than it indicates on it's own.
    // Although optimization may not happen... <https://stackoverflow.com/questions/59257543/when-is-tail-recursion-guaranteed-in-rust>
    // A mutable pointer to an enumerated iter with elements
    let lines = input.lines();
    let mut head = None;

let r_num= regex!(r"\d+");
assert_eq!(r_num.is_match("Saa"), false);
    
        for l in lines {
        let num = l.chars().enumerate()
        .filter(|(_, c)| c.is_digit(10));
    } 

    let parsed_lines: Vec<_> = lines.map(|l| l.split('.').enumerate()
        .filter(|(_, s)| !s.is_empty())
        .map(|(n, s)| (n+s.len(), s) )
    )
        .collect();
    parsed_lines.windows(2).try_fold(0usize, |sum, parsed_pair| {
        // If we can parse there is no adjacent symbol on this line.
        for item in parsed_pair[0]{
          if let Ok(num) = item.parse::<usize>(){
                if parsed_pair[1] 
            };  
        }
        return Ok(sum);
    });
    
    let parsed_lines: &[&str] = lines.try_for_each(|line| 
        {
            line.split('.').for_each( |sub| {
                    if let Ok(num) =  sub.parse::<usize>(){
                        num
                    }
            }
                )
                .map_err(|_| AocError::BadInput)?
        }
    );
    // Each line can be split on '.' to yield an array with *each comma* seporating items being a period in the original str:
    // ```
    // let v: Vec<_> = ".35.?..633../42.4445....".split('.').collect();
    // assert_eq!(v, ["", "35", "?", "", "633", "", "/42", "4445", "", "", "", ""]);
    // ```
    // Thus we can count position of substrings by it's position in the array to compair between lines

    lines
        .try_fold(0usize, |sum, line| {
            let top = line
                .split('.').for_each( |sub| 
                    //if we can parse then there is no adjacent symbol on this line.
                    if let Ok(num) =  sub.parse::<usize>(){
                        num
                    }
                )
                .map_err(|_| AocError::MissingId)?;

            let top = line
                .match_indices(|c: char| c.is_digit(10))
                .match_indices(|c: char| c != '.' && c.is_ascii_punctuation())
                .map_err(|_| AocError::MissingId)?;

            {
                return Ok(sum + id);
            }
            Ok(sum)
        })
        .map(|total| total.to_string())
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
