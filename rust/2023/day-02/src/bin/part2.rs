fn main() {
    let input = include_str!("../puzzle-input.txt");
    let output: usize = input.lines().map(parse_min_game_cube_power).sum();
    println!("Output: {}", output);
}

fn parse_min_game_cube_power(line: &str) -> usize {
    let parts: Vec<&str> = line.split(":").collect();
    let games: Vec<&str> = parts[1].split(";").collect();

    let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);

    for game in games {
        let cubes: Vec<&str> = game.split(",").collect();

        for cube in cubes {
            let (count_str, color) = cube.trim().split_once(" ").unwrap();

            let count: usize = count_str.parse().unwrap();
            match color {
                "red" => min_red = count.max(min_red),
                "green" => min_green = count.max(min_green),
                "blue" => min_blue = count.max(min_blue),
                _ => (),
            }
        }
    }

    min_red * min_green * min_blue
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

        let result: usize = input.lines().map(parse_min_game_cube_power).sum();
        assert_eq!(result, 2286);
    }
}
