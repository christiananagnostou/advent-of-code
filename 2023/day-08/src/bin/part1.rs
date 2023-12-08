use std::collections::HashMap;

fn parse_input(input: &str) -> (String, HashMap<String, Vec<String>>) {
    let mut lines = input.lines().map(|line| line.trim());
    let directions = lines.next().unwrap().to_string();
    let mut map = HashMap::new();

    for line in lines.skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(" = ").collect();
        let key = parts[0].to_string();
        let value = parts[1]
            .replace("(", "")
            .replace(")", "")
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        map.insert(key, value);
    }

    (directions, map)
}

fn calculate_steps(directions: &str, map: &HashMap<String, Vec<String>>) -> u64 {
    let mut steps = 0;
    let mut curr = "AAA".to_string();

    loop {
        let direction = directions.as_bytes()[steps as usize % directions.len()] as char;
        let next = map.get(&curr).unwrap()[if direction == 'L' { 0 } else { 1 }].clone();
        steps += 1;

        if next == "ZZZ" {
            break;
        }

        curr = next;
    }

    steps
}

fn main() {
    let input = include_str!("../puzzle-input.txt");
    let (directions, map) = parse_input(&input);
    let steps = calculate_steps(&directions, &map);

    println!("{}", steps); // 18113
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

        let (directions, map) = parse_input(&input);
        let steps = calculate_steps(&directions, &map);

        assert_eq!(steps, 6);
    }
}
