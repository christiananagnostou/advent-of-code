fn parse_input(input: &str) -> (usize, usize) {
    let mut lines = input.lines().filter(|l| !l.trim().is_empty());

    let times: usize = lines
        .next()
        .ok_or("Input is empty")
        .and_then(|line| {
            line.split_whitespace()
                .skip(1)
                .flat_map(|n| n.chars().filter(|&c| c.is_digit(10)))
                .collect::<String>()
                .parse()
                .map_err(|_| "Failed to parse times")
        })
        .unwrap_or(0);

    let distances: usize = lines
        .next()
        .ok_or("Input has only one line")
        .and_then(|line| {
            line.split_whitespace()
                .skip(1)
                .flat_map(|n| n.chars().filter(|&c| c.is_digit(10)))
                .collect::<String>()
                .parse()
                .map_err(|_| "Failed to parse distances")
        })
        .unwrap_or(0);

    (times, distances)
}

fn count_wins((t, d): &(usize, usize)) -> usize {
    let discriminant = (t * t).saturating_sub(4 * d);

    let sqrt_disc = (discriminant as f64).sqrt() as usize;
    let x1 = (t + sqrt_disc) / 2;
    let x2 = (t - sqrt_disc) / 2;

    x1 - x2
}

fn main() {
    let input = include_str!("../puzzle-input.txt");

    let race = parse_input(input);

    println!("{:?}", race);

    let win_prod = count_wins(&race);

    println!("{}", win_prod);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "
        Time:      7  15   30
        Distance:  9  40  200";

        let race = parse_input(input);

        assert_eq!(race, (71530, 940200));

        let win_prod = count_wins(&race);

        assert_eq!(win_prod, 71503);
    }
}
