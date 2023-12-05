use std::collections::HashMap;

fn count_line_points(line: &str, card_copy_counts: &mut HashMap<u32, u32>) {
    if line.is_empty() {
        return;
    }

    if let Some(card_parts) = line.split_once(": ") {
        let card_num: u32 = card_parts.0.split(' ').last().unwrap().parse().unwrap();
        let (need, have) = card_parts.1.split_once(" | ").unwrap();

        let need_map: HashMap<i32, _> = need
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok().map(|n| (n, 1)))
            .collect();

        let card_points = have
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .filter(|&num| need_map.contains_key(&num))
            .count() as u32;

        let copies = *card_copy_counts.entry(card_num).or_insert(1);

        for _ in 0..copies {
            for copy_card_num in (card_num)..(card_num + card_points) {
                *card_copy_counts.entry(copy_card_num + 1).or_insert(1) += 1;
            }
        }
    }
}

fn main() {
    let input = include_str!("../puzzle-input.txt");

    let mut card_copy_counts: HashMap<u32, u32> = HashMap::new();
    for line in input.lines() {
        count_line_points(line.trim(), &mut card_copy_counts);
    }

    let p2_total: u32 = card_copy_counts.values().sum();

    println!("{}", p2_total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let input = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let mut card_copy_counts: HashMap<u32, u32> = HashMap::new();
        for line in input.lines() {
            count_line_points(line.trim(), &mut card_copy_counts);
        }

        let p2_total: u32 = card_copy_counts.values().sum();

        assert_eq!(p2_total, 30);
    }
}
