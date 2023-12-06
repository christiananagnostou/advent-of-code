use std::cmp::{max, min};

#[derive(Debug, Clone)]
struct Mapping {
    dst: usize,
    src: usize,
    rng: usize,
}

fn apply_mappings(
    mut seeds: Vec<(usize, usize)>,
    mapping_sets: &[Vec<Mapping>],
) -> Vec<(usize, usize)> {
    // Iterate over each set of mappings
    for mappings in mapping_sets {
        let mut new_ranges = Vec::new();

        // Process each seed range
        while let Some((s, e)) = seeds.pop() {
            // Flag to track if the seed range is matched by any mapping
            let mut matched = false;

            // Check against each mapping in the current set
            for Mapping { dst, src, rng } in mappings {
                let os = max(s, *src);
                let oe = min(e, src + rng);

                // If there is an overlap, adjust the range and update seeds
                if os < oe {
                    new_ranges.push((os - src + dst, oe - src + dst));

                    if os > s {
                        seeds.push((s, os));
                    }
                    if e > oe {
                        seeds.push((oe, e));
                    }

                    // Mark as matched and exit the loop
                    matched = true;
                    break;
                }
            }

            // If no mapping matched, keep the original seed range
            if !matched {
                new_ranges.push((s, e));
            }
        }

        // Update seeds for the next iteration
        seeds = new_ranges.clone();
    }

    seeds
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

fn generate_range_vector(vector: &[usize]) -> Vec<(usize, usize)> {
    let seed_ranges: Vec<(usize, usize)> = vector
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect();

    // Ranges are inclusive on the left and exclusive on the right
    seed_ranges
}

fn main() {
    let input = include_str!("../puzzle-input.txt");

    let (seeds, mapping_sets) = parse_input(input);
    let seeds = generate_range_vector(&seeds);
    let result = apply_mappings(seeds, &mapping_sets);

    println!("{:?}", result.iter().min().unwrap().0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
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

        let (seeds, mapping_sets) = parse_input(input);
        let seeds = generate_range_vector(&seeds);
        let result = apply_mappings(seeds, &mapping_sets);

        assert_eq!(result.iter().min().unwrap().0, 46);
    }
}
