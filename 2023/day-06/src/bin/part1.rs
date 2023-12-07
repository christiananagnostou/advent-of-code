fn parse_input(input: &str) -> Result<Vec<(usize, usize)>, &'static str> {
    let mut lines = input.lines().filter(|l| !l.trim().is_empty());

    let times = lines
        .next()
        .ok_or("Input is empty")?
        .split_whitespace()
        .skip(1)
        .filter_map(|n| n.parse().ok())
        .collect::<Vec<usize>>();

    let distances = lines
        .next()
        .ok_or("Input has only one line")?
        .split_whitespace()
        .skip(1)
        .filter_map(|n| n.parse().ok())
        .collect::<Vec<usize>>();

    if times.len() != distances.len() {
        return Err("Mismatched number of times and distances");
    }

    Ok(times.into_iter().zip(distances.into_iter()).collect())
}

fn count_wins((t, d): &(usize, usize)) -> usize {
    for x in 1..(t - 1) / 2 {
        if x * (t - x) > *d {
            return t - 1 - ((x - 1) * 2);
        }
    }
    0
}

fn main() {
    let input = include_str!("../puzzle-input.txt");

    if let Ok(races) = parse_input(input) {
        let win_prod = races.iter().map(|race| count_wins(race)).product::<usize>();
        println!("Part 1: {}", win_prod);
    } else {
        println!("Failed to parse input");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "
        Time:      7  15   30
        Distance:  9  40  200";

        let races = parse_input(input).unwrap();

        assert_eq!(races, vec![(7, 9), (15, 40), (30, 200)]);

        let win_prod = races.iter().map(|race| count_wins(race)).product::<usize>();

        assert_eq!(win_prod, 288);
    }
}
