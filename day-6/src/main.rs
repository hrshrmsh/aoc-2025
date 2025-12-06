use std::fs;

// cfg_attr only derives in debug/testing
// there's a crate called derive_more but it's a bit out of scope
#[derive(PartialEq, Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
enum Operation {
    Add,
    Multiply,
}

// look way down - you'll see why this data needs to be compressed
#[derive(PartialEq, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
struct Equation {
    operation: Operation,
    operands: Vec<usize>,
}

impl Equation {
    fn new(operation: Operation, input_grid: &Vec<Vec<char>>) -> Self {
        let mut operands: Vec<usize> = vec![];

        let mut col = 0;
        while col < input_grid[0].len() {
            let mut row = 0;
            let mut base = 1;
            let mut val = 0;

            // reverse indexing when usize stops at 0 is surprisingly nuanced
            while row < input_grid.len() {
                // so it's converted down here instead
                let rev_row = input_grid.len() - row - 1;
                let ch = input_grid[rev_row][col];
                if ch != ' ' {
                    let digit = (ch as u8) - b'0';
                    val += (digit as usize) * base;
                    base *= 10;
                }

                // and row is still incremented
                row += 1;
            }

            operands.push(val);
            col += 1;
        }

        Self {
            operation,
            operands,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("error reading file");
    let (operands, operators) = read_input(&input);
    let total = sum_problems(&operands, &operators);

    println!("first total: {total}");

    let equations = read_equations(&input);
    let new_total = solve_equations(&equations);

    // first try success!
    println!("second total: {new_total}");
}

// funnily enough this part felt really simple compared to parsing the input
fn solve_equations(equations: &Vec<Equation>) -> usize {
    equations
        .iter()
        .map(|equation| match equation.operation {
            Operation::Add => equation.operands.iter().sum::<usize>(),
            Operation::Multiply => equation
                .operands
                .iter()
                .map(|c| *c)
                .reduce(|t, v| t * v)
                .expect("invalid equation"),
        })
        .sum()
}

fn sum_problems(operands: &Vec<Vec<usize>>, operators: &Vec<Operation>) -> usize {
    let mut total = 0;

    let mut index = 0;
    while index < operators.len() {
        let mut partial_sum = operands[0][index];
        for operand in &operands[1..] {
            match operators[index] {
                Operation::Add => partial_sum += operand[index],
                Operation::Multiply => partial_sum *= operand[index],
            }
        }
        total += partial_sum;
        index += 1;
    }
    total
}

fn read_input(input: &str) -> (Vec<Vec<usize>>, Vec<Operation>) {
    let mut operands = Vec::new();
    let mut operators = Vec::new();

    // I'm genuinely impressed you can just chain everything together like this
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|symbol| symbol.to_string())
                .collect::<Vec<String>>()
        })
        .for_each(|symbols| match symbols.get(0) {
            Some(operation) if "+*".contains(operation) => operators.extend(
                symbols
                    .into_iter()
                    .map(|symbol| {
                        if symbol == "+" {
                            Operation::Add
                        } else if symbol == "*" {
                            Operation::Multiply
                        } else {
                            panic!("invalid symbol")
                        }
                    })
                    .collect::<Vec<Operation>>(),
            ),
            _ => operands.push(
                symbols
                    .into_iter()
                    .map(|symbol| symbol.parse::<usize>().expect("invalid integer"))
                    .collect::<Vec<usize>>(),
            ),
        });

    (operands, operators)
}

// commented this slightly more since it feels kinda convoluted to me
// could refactor into parts but this is just a one off coding challenge
fn read_equations(input: &str) -> Vec<Equation> {
    let input_grid = input
        .lines()
        .map(|line| {
            // keep spacing this time
            line.trim_matches(|c| c == '\n' || c == '\r')
                .chars()
                .collect::<Vec<char>>()
        })
        .filter(|v| v.len() > 0)
        .collect::<Vec<Vec<char>>>();

    // to explain the algorithm:
    // the easiest consistent way to find the spacing between columns
    // is by checking where each operator symbol is since they're always
    // aligned with the start of a number column
    let operator_chars = input_grid.last().expect("grid too small");
    let mut operators = vec![];
    let mut operands = vec![];

    // so here i jumps erratically based on the indexes of each operator
    let mut i = 0;
    while i < operator_chars.len() {
        match operator_chars.get(i) {
            Some('+') => operators.push(Operation::Add),
            Some('*') => operators.push(Operation::Multiply),
            Some(_) => panic!("reached space / unknown character"),
            None => break,
        };

        // and this is why we have this section to find the next operator
        let next_i = operator_chars[i + 1..]
            .iter()
            .position(|c| *c == '*' || *c == '+')
            .unwrap_or(operator_chars.len() - i);

        // then create a 2d vec of all of the characters to process later
        // in my defense - I did not expect this to work, but it does
        // a serious implementation would be written normally
        // and not functionally - but that's part of the charm of learning
        operands.push(
            input_grid[..input_grid.len() - 1]
                .iter()
                .map(|operand_input| {
                    operand_input[i..i + next_i]
                        .iter()
                        .map(|c| *c)
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>(),
        );

        // yes it was off by 1 the first time
        // it's because position is from operator_chars[i + 1..]
        // so apply another + 1 must be applied to offset i properly
        i += next_i + 1;
    }

    // even I could not stomach returning a tuple with a triple-vec of chars
    // so it was around here when I finally started using structs
    let mut equations = vec![];
    let mut i = 0;
    while i < operands.len() {
        // scroll up to the equation impl but it's nothing novel
        equations.push(Equation::new(operators[i], &operands[i]));
        i += 1;
    }
    equations
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_input() {
        let example_text = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let expected_operands = vec![
            vec![123, 328, 51, 64],
            vec![45, 64, 387, 23],
            vec![6, 98, 215, 314],
        ];
        let expected_operators = vec![
            Operation::Multiply,
            Operation::Add,
            Operation::Multiply,
            Operation::Add,
        ];

        let (operands, operators) = read_input(example_text);

        assert_eq!(expected_operands, operands);
        assert_eq!(expected_operators, operators);
    }

    #[test]
    fn test_read_input_two() {
        let example_text = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let expected_equations = vec![
            Equation {
                operation: Operation::Multiply,
                operands: vec![1, 24, 356],
            },
            Equation {
                operation: Operation::Add,
                operands: vec![369, 248, 8],
            },
            Equation {
                operation: Operation::Multiply,
                operands: vec![32, 581, 175],
            },
            Equation {
                operation: Operation::Add,
                operands: vec![623, 431, 4],
            },
        ];

        let equations = read_equations(example_text);
        assert_eq!(equations, expected_equations);
    }

    #[test]
    fn example_part_one() {
        let example_text = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let (operands, operators) = read_input(example_text);

        assert_eq!(sum_problems(&operands, &operators), 4277556);
    }

    #[test]
    fn example_part_two() {
        let example_text = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let equations = read_equations(&example_text);

        // proud to say this worked first time
        assert_eq!(solve_equations(&equations), 3263827);
    }
}
