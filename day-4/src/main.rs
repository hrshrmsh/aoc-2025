use std::fs;

fn main() {
    let input_str = fs::read_to_string("input.txt").expect("?");
    let mut grid = read_input(&input_str);

    println!(
        "pass 1 accessible rolls: {}",
        count_accessible_rolls(&grid).iter().count()
    );
    println!("total rolls removed: {}", total_rolls(&mut grid));
}

fn total_rolls(grid: &mut Vec<Vec<usize>>) -> usize {
    let mut total = 0;
    loop {
        let result = count_accessible_rolls(&grid);
        let valid = result.iter().count();
        total += valid;
        println!("found {} valid rolls", valid);

        if valid == 0 {
            break;
        } else {
            for (i, j) in result {
                grid[i][j] = 0;
            }
        }
    }
    total
}

fn count_accessible_rolls(grid: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut valid = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 1 {
                // usize must be > 0 and they're used to index into collections
                let min_i = if i > 0 { i - 1 } else { i };
                let min_j = if j > 0 { j - 1 } else { j };
                let mut adjacent = 0;

                // min_j == j when j == 0, so there is no need to check it
                if min_j != j && row.get(min_j).is_some_and(|x| *x == 1) {
                    adjacent += 1;
                }
                if row.get(j + 1).is_some_and(|x| *x == 1) {
                    adjacent += 1;
                }
                // you can chain if logic w/ lets too apparently
                if min_i != i
                    && let Some(row_above) = grid.get(min_i)
                {
                    for k in min_j..=j + 1 {
                        if row_above.get(k).is_some_and(|x| *x == 1) {
                            adjacent += 1;
                        }
                    }
                }
                if let Some(row_below) = grid.get(i + 1) {
                    for k in min_j..=j + 1 {
                        if row_below.get(k).is_some_and(|x| *x == 1) {
                            adjacent += 1;
                        }
                    }
                }

                if adjacent < 4 {
                    valid.push((i, j));
                }
            } else if *val == 7 {
                panic!("hey! the grid isn't valid!");
            }
        }
    }

    valid
}

// in retrospect the conversion isn't necessary
// but it's convenient
fn read_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => 0,
                    '@' => 1,
                    _ => 7, // shouldn't reach this
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_input_reading() {
        let example_text = "@@@@@\n.....\n@@..@";
        let expected_vec = vec![vec![1; 5], vec![0; 5], vec![1, 1, 0, 0, 1]];
        let test_vec = read_input(example_text);

        assert_eq!(test_vec, expected_vec);
    }

    #[test]
    fn example_part_one() {
        let example_text = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let parsed_grid = read_input(example_text);

        assert_eq!(count_accessible_rolls(&parsed_grid).iter().count(), 13);
    }

    #[test]
    fn example_part_two() {
        let example_text = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let mut parsed_grid = read_input(example_text);

        assert_eq!(total_rolls(&mut parsed_grid), 43);
    }
}
