//! # Day 3: Gear Ratios - Part 1
//!
//! The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.
//! You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.
//! Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.
//! The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.
//! This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.
//! Consider the same engine schematic again:use crate::custom_error::AocError;
//! ```
//!
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
//! In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.
//!
//! **What is the sum of all of the gear ratios in your engine schematic?**

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
    // ASSUME: All lines are same length!
    let line_len = input.find('\n').ok_or(AocError::BadInput)?;

    // Collect the byte index of each punctuation, this will be in incremental order, unless latter done on threads...
    let mut punct_pos = Vec::with_capacity(input.len() / 20);
    // Collect the index range for every number, and the value within
    let mut digit_indicies = Vec::with_capacity(input.len() / 4);

    let mut buf: Option<(usize, String)> = None;
    for (idx, c) in input.chars().enumerate() {
        if c.is_ascii_digit() {
            if buf.is_none() {
                buf = Some((idx, c.to_string()));
                continue;
            }
            buf.as_mut().unwrap().1.push(c);
            continue;
        }
        if buf.is_some() {
            // we are one past the number, ala Range (start..end
            // std::ops::Range {start: buf.0.unwrap(), end: idx}
            digit_indicies.push(buf.unwrap());
            buf = None;
        }
        if c == '*' {
            punct_pos.push(idx);
            continue;
        }
    }

    dbg!(punct_pos.clone());
    dbg!(digit_indicies.clone());

    let mut total = 0;
    for p in punct_pos {
        // find adjacent punctutation to any parsed number
        for (idx, num) in digit_indicies.clone() {
            let diff = p as isize - idx as isize;
            if diff.unsigned_abs() > num.len() + line_len + 1 {
                continue;
            }
            println!("p - idx == diff // {p} - {idx} == {diff}");

            if diff == -1 // char before
            || diff == num.len() as isize // char after
            || (-(line_len as isize + 2)..= -((line_len - num.len() +1 ) as isize )).contains(&diff)
            || ((line_len as isize )..= ((line_len + num.len() + 1)as isize)).contains(&diff)
            // FIXME: ^^ the above should just be a abs val check... range line before and after
            {
                total += dbg!(num.parse::<usize>().map_err(|_| AocError::CannotParse)?);
                println!("TOTAL: {total}");
            }
        }
        // We want to prune too far back items, as we are sorted,
        // anything past 2 chars + earliest puctuation + line length is imposssible to be adjacent.
        while p.checked_sub(line_len + digit_indicies[0].0 + 2).is_some() {
            digit_indicies.drain(0..1);
        }
    }

    Ok(total.to_string())
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
.664.598..
";
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
