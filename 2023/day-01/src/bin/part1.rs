fn main() {
    let input = include_str!("../puzzle-input.txt");
    let output: usize = input.lines().map(parse_calibration_value).sum();
    println!("Output: {}", output);
}

fn parse_calibration_value(line: &str) -> usize {
    let first_digit = line.chars().find(|c| c.is_numeric()).unwrap();
    let last_digit = line.chars().rfind(|c| c.is_numeric()).unwrap();
    format!("{first_digit}{last_digit}").parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let result: usize = input.lines().map(parse_calibration_value).sum();
        assert_eq!(result, 142);
    }
}
