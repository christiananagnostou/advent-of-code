fn main() {
    let input = include_str!("../puzzle-input.txt");

    let mut input_matrix = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let output: usize = count_part_numbers(&mut input_matrix);

    println!("Output: {}", output);
}

fn is_symbol(c: char) -> bool {
    !c.is_whitespace() && c != '.' && !c.is_numeric()
}

fn kill_all_surrounding_numbers(matrix: &mut Vec<Vec<char>>, row: usize, col: usize) {
    for i in (row.saturating_sub(1))..=(row + 1) {
        for j in (col.saturating_sub(1))..=(col + 1) {
            if i >= matrix.len() || j >= matrix[i].len() || matrix[i][j] == '.' {
                continue;
            }
            if matrix[i][j].is_numeric() {
                kill_lateral_numbers(matrix, i, j)
            }
            matrix[i][j] = '.';
        }
    }
}

// Recurrisive function to kill numbers to the left and right of given coordinates
fn kill_lateral_numbers(matrix: &mut Vec<Vec<char>>, row: usize, col: usize) {
    // Kill number to the left
    if col > 0 && matrix[row][col.saturating_sub(1)].is_numeric() {
        matrix[row][col - 1] = '.';
        kill_lateral_numbers(matrix, row, col - 1);
    }

    // Kill number to the right
    if col < matrix[row].len() - 1 && matrix[row][col + 1].is_numeric() {
        matrix[row][col + 1] = '.';
        kill_lateral_numbers(matrix, row, col + 1);
    }
}

fn count_matrix_numbers(matrix: &Vec<Vec<char>>) -> usize {
    let mut sum = 0usize;

    for row in matrix {
        let mut number = String::new();
        for &ch in row {
            if ch.is_numeric() {
                number.push(ch);
            } else if !number.is_empty() {
                sum += number.parse::<usize>().unwrap();
                number.clear();
            }
        }
        if !number.is_empty() {
            sum += number.parse::<usize>().unwrap();
        }
    }

    sum
}

fn count_part_numbers(matrix: &mut Vec<Vec<char>>) -> usize {
    let all_parts = count_matrix_numbers(matrix);

    for (row_idx, row) in matrix.clone().iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if is_symbol(*col) {
                kill_all_surrounding_numbers(matrix, row_idx, col_idx)
            }
        }
    }

    let bad_parts = count_matrix_numbers(matrix);
    all_parts - bad_parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
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

        let mut input_matrix = input
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect::<Vec<Vec<char>>>();

        let result: usize = count_part_numbers(&mut input_matrix);

        assert_eq!(result, 4361);
    }
}
