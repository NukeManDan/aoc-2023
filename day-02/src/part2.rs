//! # Day 2: Cube Conundrum -- Part 2
//!
//! The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself. It's just up ahead!
//!
//! As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of cubes of each color that could have been in the bag to make the game possible?
//!
//! Again consider the example games from earlier:
//!
//! ```
//! Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
//! Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
//! Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
//! Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
//! Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
//! ```
//!
//! - In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
//! - Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
//! - Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
//! - Game 4 required at least 14 red, 3 green, and 15 blue cubes.
//! - Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
//!
//! The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces the sum 2286.
//!
//! For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    input
        .lines()
        .try_fold(0usize, |sum, line| {
            let mut sets = line.split(&[':', ';'][..]);

            // First we have all sets iters starting with the game ID:
            sets.next()
                .expect("line must start with \"Game <ID>:\"")
                .strip_prefix("Game ")
                .ok_or(AocError::MissingId)?
                .parse::<usize>()
                .map_err(|_| AocError::MissingId)?;

            let mut red_max = 0;
            let mut green_max = 0;
            let mut blue_max = 0;

            if sets
                .try_for_each(|set| {
                    for ball in set.split(',') {
                        if let Some(red) = ball.strip_suffix("red") {
                            let red_count = red
                                .trim()
                                .parse::<usize>()
                                .map_err(|_| AocError::SetMalformed)?;
                            if red_count > red_max {
                                red_max = red_count;
                            };
                        };

                        if let Some(green) = ball.strip_suffix("green") {
                            let green_count = green
                                .trim()
                                .parse::<usize>()
                                .map_err(|_| AocError::SetMalformed)?;
                            if green_count > green_max {
                                green_max = green_count;
                            };
                        };

                        if let Some(blue) = ball.strip_suffix("blue") {
                            let blue_count = blue
                                .trim()
                                .parse::<usize>()
                                .map_err(|_| AocError::SetMalformed)?;
                            if blue_count > blue_max {
                                blue_max = blue_count;
                            };
                        };
                    }
                    Ok::<(), AocError>(())
                })
                .is_ok()
            {
                return Ok(sum + (red_max * green_max * blue_max));
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
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
