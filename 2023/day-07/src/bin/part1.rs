use std::{cmp::Ordering, collections::HashMap};

fn split_hand(input: &str) -> (&str, &str) {
    let mut parts = input.split_whitespace();
    let cards = parts.next().unwrap_or("");
    let points = parts.next().unwrap_or("");

    (cards, points)
}

fn char_frequencies(hand: &str) -> Vec<usize> {
    let mut freq = HashMap::new();
    for c in hand.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    let mut frequencies: Vec<usize> = freq.values().cloned().collect();
    frequencies.sort_by(|a, b| b.cmp(a)); // Sort in descending order
    frequencies
}

fn rank_hand(hand: &str) -> usize {
    let hand_freqs = char_frequencies(hand);

    if hand_freqs[0] == 5 {
        return 6;
    } else if hand_freqs[0] == 4 {
        return 5;
    } else if hand_freqs[0] == 3 && hand_freqs[1] == 2 {
        return 4;
    } else if hand_freqs[0] == 3 {
        return 3;
    } else if hand_freqs[0] == 2 && hand_freqs[1] == 2 {
        return 2;
    } else if hand_freqs[0] == 2 {
        return 1;
    } else {
        0
    }
}

fn card_to_points(card: char) -> Option<u32> {
    let mut points = HashMap::new();
    points.insert('A', 14);
    points.insert('K', 13);
    points.insert('Q', 12);
    points.insert('J', 11);
    points.insert('T', 10);
    points.insert('9', 9);
    points.insert('8', 8);
    points.insert('7', 7);
    points.insert('6', 6);
    points.insert('5', 5);
    points.insert('4', 4);
    points.insert('3', 3);
    points.insert('2', 2);

    points.get(&card).cloned()
}

fn compare_letters(hand1: &str, hand2: &str) -> Ordering {
    for (c1, c2) in hand1.chars().zip(hand2.chars()) {
        if c1 != c2 {
            if let (Some(p1), Some(p2)) = (card_to_points(c1), card_to_points(c2)) {
                return p1.cmp(&p2);
            }
        }
    }

    Ordering::Equal
}

fn compare_hands(a: &str, b: &str) -> Ordering {
    let (a_cards, _) = split_hand(a);
    let (b_cards, _) = split_hand(b);

    let a_rank = rank_hand(a_cards);
    let b_rank = rank_hand(b_cards);

    if a_rank == b_rank {
        compare_letters(a, b)
    } else {
        a_rank.cmp(&b_rank)
    }
}

fn calculate_score(sorted_hands: Vec<&str>) -> usize {
    sorted_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            let (_, points) = split_hand(hand);
            let parsed_points: usize = points.parse().unwrap_or(0);
            (i + 1) * parsed_points
        })
        .sum()
}

fn main() {
    let input = include_str!("../puzzle-input.txt");

    let mut sorted_hands: Vec<&str> = input.lines().map(|line| line.trim()).collect();

    sorted_hands.sort_by(|a, b| compare_hands(a, b));

    let score = calculate_score(sorted_hands);

    println!("{:?}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        let mut sorted_hands: Vec<&str> = input.lines().map(|line| line.trim()).collect();

        sorted_hands.sort_by(|a, b| compare_hands(a, b));

        let score = calculate_score(sorted_hands);

        assert_eq!(score, 6440);
    }
}
