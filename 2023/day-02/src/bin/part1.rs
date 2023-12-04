const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn main() {
    let input = include_str!("../puzzle-input.txt");
    let output: usize = input.lines().map(parse_valid_game_ids).sum();
    println!("Output: {}", output);
}

fn parse_valid_game_ids(line: &str) -> usize {
    let parts: Vec<&str> = line.split(":").collect();
    let game_id = parts.get(0).unwrap().split(" ").last().unwrap();
    let games: Vec<&str> = parts.get(1).unwrap().split(";").collect();

    for game in &games {
        let cubes: Vec<&str> = game.split(",").collect();

        for cube in &cubes {
            let (count_str, color) = cube.trim().split_once(" ").unwrap();
            let count: usize = count_str.parse().unwrap();

            match color {
                "red" if count > MAX_RED => return 0,
                "green" if count > MAX_GREEN => return 0,
                "blue" if count > MAX_BLUE => return 0,
                _ => (),
            }
        }
    }

    game_id.parse().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result: usize = input.lines().map(parse_valid_game_ids).sum();
        assert_eq!(result, 8);
    }
}
