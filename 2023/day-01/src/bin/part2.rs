fn main() {
    let input = include_str!("../puzzle-input.txt");
    let output: usize = input.lines().map(parse_calibration_value).sum();
    println!("Output: {}", output);
}

fn parse_calibration_value(line: &str) -> usize {
    let line = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    let first_digit = line.chars().find(|c| c.is_digit(10)).unwrap();
    let last_digit = line.chars().rfind(|c| c.is_digit(10)).unwrap();
    format!("{first_digit}{last_digit}").parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        sevenine
        eighthree";

        let result: usize = input.lines().map(parse_calibration_value).sum();
        assert_eq!(result, 443);
    }
}
