use crate::custom_error::AocError;
use miette::Result;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    input
        .lines()
        .try_fold(0usize, |sum, line| {
            let left = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .ok_or(AocError::DigitMissing)?;
            let right = line
                .chars()
                .rfind(|c| c.is_ascii_digit())
                .ok_or(AocError::DigitMissing)?;
            Ok(sum
                + String::from_iter([left, right])
                    .parse::<usize>()
                    .map_err(|_| AocError::SumError)?)
        })
        .map(|total| total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
