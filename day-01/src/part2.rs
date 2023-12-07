//! Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
//!
//! Equipped with this new information, you now need to find the real first and last digit on each line. For example:
//!
//! ```
//! two1nine
//! eightwothree
//! abcone2threexyz
//! xtwone3four
//! 4nineeightseven2
//! zoneight234
//! 7pqrstsixteen
//! ```
//!
//! In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

use nom::AsChar;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // NOTE: We must STEAM each line and get the FIRST word -> number conversion (left to right).
    // It could be builtins impl a search/replace fn that is not l->r explicitly and some may even
    // find/replace overlaping words!
    // Example: "fiveight" - this should map to 58.
    //
    // Words we search for are (3..=5) char long, so we can:
    // 1. scan for number chars (0..9)
    // 2. scan for substr that are before first or after last digit, iff position is > 3 char len.
    // but we want one read/alloc per line ... so why not stream chars:
    // 1.
    // 2.
    // 3.
    input
        .lines()
        .try_fold(0usize, |sum, line| {
            let left = {
                let maybe_digit_position = line.find(|c: char| c.is_ascii_digit());

                let maybe_number = match maybe_digit_position {
                    Some(pos) => find_number_text(
                        line.get(0..pos)
                            .expect("we get a UTF8 substring, even if (0..0) => length = 0"),
                    ),
                    _ => find_number_text(line),
                };

                match maybe_number {
                    Some(ch) => ch,
                    // FIXME: this will behave badly if not ASCII, as it demands UTF8 encoded strings!
                    None => line.as_bytes()[maybe_digit_position.ok_or(AocError::DigitMissing)?]
                        .as_char(),
                }
            };
            let right = {
                let maybe_digit_position = line.rfind(|c: char| c.is_ascii_digit());

                let maybe_number = match maybe_digit_position {
                    Some(pos) => rfind_number_text(
                        line.get(pos..line.len())
                            .expect("we get a UTF8 substring, even if (0..0) => length = 0"),
                    ),
                    _ => rfind_number_text(line),
                };

                match maybe_number {
                    Some(ch) => ch,
                    // FIXME: this will behave badly if not ASCII, as it demands UTF8 encoded strings!
                    None => line.as_bytes()[maybe_digit_position.ok_or(AocError::DigitMissing)?]
                        .as_char(),
                }
            };
            Ok(sum
                + String::from_iter([left, right])
                    .parse::<usize>()
                    .map_err(|_| AocError::SumError)?)
        })
        .map(|total| total.to_string())
}

/// Find the FIRST number fully written out in English (set "one",...,"nine") - left to right -  in a string literal.
/// Return it's `char` representation (set '1'..'9').
pub fn find_number_text(line_slice: &str) -> Option<char> {
    if line_slice.len() < 3 {
        return None;
    };

    let mut maybe_best: Option<(usize, char)> = None;

    //step by 3, width of 5 chars to scan completely, if window of 5 is too big, do only 3 or 4 char words find.

    if let Some(pos) = line_slice.find("one") {
        if let Some(best) = maybe_best {
            if best.0 > pos {
                maybe_best = Some((pos, '1'))
            };
        } else {
            maybe_best = Some((pos, '1'))
        }
    };
    if let Some(pos) = line_slice.find("two") {
        if let Some(best) = maybe_best {
            if best.0 > pos {
                maybe_best = Some((pos, '2'))
            };
        } else {
            maybe_best = Some((pos, '2'))
        }
    };
    if let Some(pos) = line_slice.find("three") {
        if let Some(best) = maybe_best {
            if best.0 > pos {
                maybe_best = Some((pos, '3'))
            };
        } else {
            maybe_best = Some((pos, '3'))
        }
    };
    if let Some(pos) = line_slice.find("four") {
        if let Some(best) = maybe_best {
            if best.0 > pos {
                maybe_best = Some((pos, '4'))
            };
        } else {
            maybe_best = Some((pos, '4'))
        }
    };
    if let Some(pos) = line_slice.find("five") {
        if let Some(best) = maybe_best {
            if best.0 > pos {
                maybe_best = Some((pos, '5'))
            };
        } else {
            maybe_best = Some((pos, '5'))
        }
    };
    if let Some(pos) = line_slice.find("six") {
        if let Some(best) = maybe_best {
            if best.0 > pos {
                maybe_best = Some((pos, '6'))
            };
        } else {
            maybe_best = Some((pos, '6'))
        }
    };
    if let Some(pos) = line_slice.find("seven") {
        if let Some(best) = maybe_best {
            if best.0 > pos {
                maybe_best = Some((pos, '7'))
            };
        } else {
            maybe_best = Some((pos, '7'))
        }
    };
    if let Some(pos) = line_slice.find("eight") {
        if let Some(best) = maybe_best {
            if best.0 > pos {
                maybe_best = Some((pos, '8'))
            };
        } else {
            maybe_best = Some((pos, '8'))
        }
    };
    if let Some(pos) = line_slice.find("nine") {
        if let Some(best) = maybe_best {
            if best.0 > pos {
                maybe_best = Some((pos, '9'))
            };
        } else {
            maybe_best = Some((pos, '9'))
        }
    };
    Some(maybe_best?.1)
}

/// Find the FIRST number fully written out in English (set "one",...,"nine") - left to right -  in a string literal.
/// Return it's `char` representation (set '1'..'9').
pub fn rfind_number_text(line_slice: &str) -> Option<char> {
    if line_slice.len() < 3 {
        return None;
    };

    let mut maybe_best: Option<(usize, char)> = None;

    //step by 3, width of 5 chars to scan completely, if window of 5 is too big, do only 3 or 4 char words find.

    if let Some(pos) = line_slice.rfind("one") {
        if let Some(best) = maybe_best {
            if best.0 < pos {
                maybe_best = Some((pos, '1'))
            };
        } else {
            maybe_best = Some((pos, '1'))
        }
    };
    if let Some(pos) = line_slice.rfind("two") {
        if let Some(best) = maybe_best {
            if best.0 < pos {
                maybe_best = Some((pos, '2'))
            };
        } else {
            maybe_best = Some((pos, '2'))
        }
    };
    if let Some(pos) = line_slice.rfind("three") {
        if let Some(best) = maybe_best {
            if best.0 < pos {
                maybe_best = Some((pos, '3'))
            };
        } else {
            maybe_best = Some((pos, '3'))
        }
    };
    if let Some(pos) = line_slice.rfind("four") {
        if let Some(best) = maybe_best {
            if best.0 < pos {
                maybe_best = Some((pos, '4'))
            };
        } else {
            maybe_best = Some((pos, '4'))
        }
    };
    if let Some(pos) = line_slice.rfind("five") {
        if let Some(best) = maybe_best {
            if best.0 < pos {
                maybe_best = Some((pos, '5'))
            };
        } else {
            maybe_best = Some((pos, '5'))
        }
    };
    if let Some(pos) = line_slice.rfind("six") {
        if let Some(best) = maybe_best {
            if best.0 < pos {
                maybe_best = Some((pos, '6'))
            };
        } else {
            maybe_best = Some((pos, '6'))
        }
    };
    if let Some(pos) = line_slice.rfind("seven") {
        if let Some(best) = maybe_best {
            if best.0 < pos {
                maybe_best = Some((pos, '7'))
            };
        } else {
            maybe_best = Some((pos, '7'))
        }
    };
    if let Some(pos) = line_slice.rfind("eight") {
        if let Some(best) = maybe_best {
            if best.0 < pos {
                maybe_best = Some((pos, '8'))
            };
        } else {
            maybe_best = Some((pos, '8'))
        }
    };
    if let Some(pos) = line_slice.rfind("nine") {
        if let Some(best) = maybe_best {
            if best.0 < pos {
                maybe_best = Some((pos, '9'))
            };
        } else {
            maybe_best = Some((pos, '9'))
        }
    };
    Some(maybe_best?.1)
}

pub fn find_position_of_digit(full: &str, sub: &str) -> Option<(usize, char)> {
    Some((
        full.find(sub)?,
        match sub {
            "one" => '1',
            "two" => '2',
            "three" => '3',
            "four" => '4',
            "five" => '5',
            "six" => '6',
            "seven" => '7',
            "eight" => '8',
            "nine" => '9',
            _ => return None,
        },
    ))
}

pub fn find_written_digit(line: &str) -> Option<char> {
    if line.len() < 3 {
        return None;
    };

    if line.starts_with("one") {
        return Some('1');
    };
    if line.starts_with("two") {
        return Some('2');
    };
    if line.starts_with("three") {
        return Some('3');
    };
    if line.starts_with("four") {
        return Some('4');
    };
    if line.starts_with("five") {
        return Some('5');
    };
    if line.starts_with("six") {
        return Some('6');
    };
    if line.starts_with("seven") {
        return Some('7');
    };
    if line.starts_with("eight") {
        return Some('8');
    };
    if line.starts_with("nine") {
        return Some('9');
    };

    // We check the next char, recursively, in the line
    find_written_digit(
        std::str::from_utf8(&line.as_bytes()[1..]).expect("must have UTF8 encoded strings"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
