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

fn get_all_a_nodes(map: &HashMap<String, Vec<String>>) -> Vec<&String> {
    map.keys().filter(|key| key.ends_with("A")).collect()
}

fn calculate_steps(
    directions: &String,
    map: &HashMap<String, Vec<String>>,
    all_a_nodes: &[&String],
) -> Vec<usize> {
    let mut steps = Vec::new();

    for key in all_a_nodes {
        let mut curr = *key;
        let mut step_count = 0;

        loop {
            let direction = directions.as_bytes()[step_count % directions.len()] as char;
            let next = &map.get(curr).unwrap()[if direction == 'L' { 0 } else { 1 }];
            step_count += 1;

            if next.chars().nth(2).unwrap() == 'Z' {
                steps.push(step_count);
                break;
            }

            curr = next;
        }
    }

    steps
}

fn calculate_lcm(steps: &[usize]) -> usize {
    steps.iter().fold(1, |acc, val| acc * val / gcd(acc, *val))
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let input = include_str!("../puzzle-input.txt");

    let (directions, map) = parse_input(&input);
    let all_a_nodes = get_all_a_nodes(&map);
    let steps = calculate_steps(&directions, &map, &all_a_nodes);
    let result = calculate_lcm(&steps);

    println!("{}", result); // 12315788159977
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        let (directions, map) = parse_input(&input);
        let all_a_nodes = get_all_a_nodes(&map);
        let steps = calculate_steps(&directions, &map, &all_a_nodes);
        let result = calculate_lcm(&steps);

        assert_eq!(result, 6);
    }
}
