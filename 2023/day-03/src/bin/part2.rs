use std::collections::HashMap;

fn is_symbol(c: char) -> bool {
    !c.is_whitespace() && c != '.' && !c.is_numeric()
}

fn process_board_characters<F>(
    board: &[String],
    mut char_callback: F,
    start_y: usize,
    start_x: usize,
    end_y: usize,
    end_x: usize,
) where
    F: FnMut(char, usize, usize),
{
    for y in start_y..=end_y {
        for x in start_x..=end_x {
            if let Some(c) = board.get(y).and_then(|r| r.chars().nth(x)) {
                char_callback(c, y, x);
            }
        }
    }
}

fn extract_gear_numbers(board: &[String]) -> (i32, i32) {
    let num_pattern = regex::Regex::new(r"\d+").unwrap();
    let mut gear_nums = HashMap::new();
    let mut p1_total = 0;

    for (row_num, line) in board.iter().enumerate() {
        for mat in num_pattern.find_iter(line) {
            let num = mat.as_str().parse::<i32>().unwrap();
            let mut has_added_num = false;

            let mut process_char = |c, y, x| {
                if is_symbol(c) {
                    gear_nums
                        .entry((y, x, c))
                        .or_insert_with(Vec::new)
                        .push(num);
                    has_added_num = has_added_num || true;
                }
            };

            process_board_characters(
                &board,
                &mut process_char,
                row_num.saturating_sub(1),
                mat.start().saturating_sub(1),
                row_num + 1,
                mat.end(),
            );

            p1_total += num * has_added_num as i32;
        }
    }

    let p2_total = gear_nums
        .iter()
        .filter(|&(&(_, _, c), _)| c == '*')
        .filter_map(|(_, v)| {
            if v.len() == 2 {
                Some(v[0] * v[1])
            } else {
                None
            }
        })
        .sum();

    (p1_total, p2_total)
}

fn main() {
    let board: Vec<String> = include_str!("../puzzle-input.txt")
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    let (p1_total, p2_total) = extract_gear_numbers(&board);

    println!("{}", p1_total);
    println!("{}", p2_total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let input = "467..114..
                           ...*......
                           ..35..633.
                           ......#...
                           617*......
                           .....+.58.
                           ..592.....
                           ......755.
                           ...$.*....
                           .664.598..";

        let input_matrix: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect();

        let (p1_total, p2_total) = extract_gear_numbers(&input_matrix);

        assert_eq!(p1_total, 4361);
        assert_eq!(p2_total, 467835);
    }
}
