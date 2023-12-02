use regex::{Regex, RegexSet};

fn main() {
    let input = include_str!("../puzzle-input.txt");
    let output = sum_calibrations(input);
    println!("Output: {}", output);
}

fn sum_calibrations(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let value = parse_calibration_value(line.trim()).unwrap_or(0);
        println!("Value: {}", value);
        sum += value;
    }
    sum
}

fn parse_calibration_value(line: &str) -> Option<i32> {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").ok()?;
    let mut captures = re.find_iter(line);

    let patterns = [
        "[0-9]", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let set = RegexSet::new(patterns).unwrap();
    // Compile each pattern independently.
    let regexes: Vec<_> = set
        .patterns()
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect();

    // Match against the whole set first and identify the individual matching patterns.
    let matches: Vec<&str> = set
        .matches(line)
        .into_iter()
        // Dereference the match index to get the corresponding compiled pattern.
        .map(|index| &regexes[index])
        // To get match locations or any other info, we then have to search the exact same haystack again, using our separately-compiled pattern.
        .map(|re| re.find(line).unwrap().as_str())
        .collect();

    println!("Matches: {:?}", set.matches(line));

    let first_digit = parse_digit(captures.next()?.as_str())?;
    let last_digit = parse_digit(captures.last()?.as_str())?;

    let concatenated = format!("{}{}", first_digit, last_digit);
    concatenated.parse().ok()
}

fn parse_digit(digit: &str) -> Option<i8> {
    digit.parse().ok().or_else(|| word_to_number(digit))
}

fn word_to_number(word: &str) -> Option<i8> {
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum_calibrations(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            eighthree
            sevenine",
        );
        assert_eq!(result, 281 + 83 + 79);
    }
}
