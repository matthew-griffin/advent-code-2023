pub fn part_one(input: &str) -> u32 {
    input.lines()
    .map(|line| {
        let first_digit = line.chars().find(char::is_ascii_digit).unwrap();
        let last_digit = line.chars().rev().find(char::is_ascii_digit).unwrap();
        let number = format!("{first_digit}{last_digit}");
        number.parse::<u32>().unwrap()
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn test_input() -> &'static str {
        indoc! {"1abc2
                pqr3stu8vwx
                a1b2c3d4e5f
                treb7uchet"}
    }

    #[test]
    fn test_part_one() {
        let result = part_one(test_input());
        assert_eq!(result, 142);
    }
}
