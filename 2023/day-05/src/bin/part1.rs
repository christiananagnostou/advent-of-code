// use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Mapping {
    dst: usize,
    src: usize,
    rng: usize,
}

fn apply_mappings(curr: usize, mappings: &Vec<Mapping>) -> usize {
    for Mapping { dst, src, rng } in mappings {
        if curr >= *src && curr < *src + *rng {
            return *dst + (curr - *src);
        }
    }
    return curr;
}

fn get_layer_index(layer: &str) -> Option<usize> {
    match layer {
        "seed-to-soil" => Some(0),
        "soil-to-fertilizer" => Some(1),
        "fertilizer-to-water" => Some(2),
        "water-to-light" => Some(3),
        "light-to-temperature" => Some(4),
        "temperature-to-humidity" => Some(5),
        "humidity-to-location" => Some(6),
        _ => None,
    }
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Vec<Mapping>>) {
    let mut seeds = Vec::new();
    let mut mappings = vec![Vec::new(); 7];
    let mut curr_mapping_layer = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() > 1 {
            match parts[0] {
                "seeds:" => {
                    seeds = parts[1..].iter().filter_map(|&x| x.parse().ok()).collect();
                }
                layer => {
                    if let Some(index) = get_layer_index(layer) {
                        curr_mapping_layer = index;
                    }
                }
            }
        }

        if parts.len() == 3 {
            let mapping_values: Vec<usize> = parts.iter().filter_map(|&x| x.parse().ok()).collect();

            let mapping = Mapping {
                dst: mapping_values[0],
                src: mapping_values[1],
                rng: mapping_values[2],
            };
            mappings[curr_mapping_layer].push(mapping);
        }
    }

    (seeds, mappings)
}

fn main() {
    let input = include_str!("../puzzle-input.txt");

    let (mut seeds, mappings) = parse_input(input);

    for seed_index in 0..seeds.len() {
        for mapping in &mappings {
            let val = apply_mappings(seeds[seed_index], mapping);
            seeds[seed_index] = val;
        }
    }

    println!("{:?}", seeds.iter().min())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4";

        let (mut seeds, mappings) = parse_input(input);

        for seed_index in 0..seeds.len() {
            for mapping in &mappings {
                let val = apply_mappings(seeds[seed_index], mapping);
                seeds[seed_index] = val;
            }
        }

        assert_eq!(seeds.iter().min(), Some(&35));
    }
}
