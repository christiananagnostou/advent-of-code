use std::collections::HashMap;

fn count_line_points(line: &str) -> u32 {
    line.split(": ")
        .nth(1)
        .map(|card_info| {
            let (winning_set, my_set) = card_info.split_once(" | ").unwrap();
            let winning_nums: HashMap<i32, _> = winning_set
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok().map(|n| (n, 1)))
                .collect();

            let card_points = my_set
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .filter(|&num| winning_nums.contains_key(&num))
                .count() as u32;

            if card_points > 0 {
                1 << (card_points - 1)
            } else {
                0
            }
        })
        .unwrap_or(0)
}

fn main() {
    let input = include_str!("../puzzle-input.txt");
    let output: u32 = input
        .lines()
        .map(|line| count_line_points(line.trim()))
        .sum();

    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let p1_total: u32 = input
            .lines()
            .map(|line| count_line_points(line.trim()))
            .sum();

        assert_eq!(p1_total, 13);
    }
}
