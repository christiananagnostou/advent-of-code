fn main() {
    let input = include_str!("../puzzle-input.txt");
    let output = sum_calibrations(input);
    println!("Output: {}", output);
}

fn sum_calibrations(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let value = parse_calibration_value(&line).unwrap_or(0);
        sum += value;
    }
    return sum;
}

fn parse_calibration_value(line: &str) -> Option<i32> {
    let first_digit = line.chars().find(|c| c.is_digit(10));
    let last_digit = line.chars().rfind(|c| c.is_digit(10));

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => {
            let concatenated = format!("{}{}", first, last);
            Some(concatenated.parse().unwrap())
        }
        _ => {
            println!("No digits found");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum_calibrations(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
