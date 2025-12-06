use std::fs;

fn main() {
    let input = read_input(&fs::read_to_string("input.txt").expect(""));
    let mut results = Vec::new();
    let mut results_p2 = Vec::new();

    for i in &input {
        let r1 = max_joltage(i);
        let r2 = ultra_joltage(&i, 12);
        println!("found joltages: {r1} | {r2}");
        results.push(r1);
        results_p2.push(r2);
    }

    println!("total joltage: {}", results.iter().sum::<usize>());
    println!("ultra joltage: {}", results_p2.iter().sum::<usize>());
}

fn read_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| (c as u8 - b'0') as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn max_joltage(power_supply: &Vec<usize>) -> usize {
    let mut max = 0;

    let mut p1 = 0;
    while p1 < power_supply.len() - 1 {
        let mut p2 = p1 + 1;
        while p2 < power_supply.len() {
            let tmp = power_supply[p1] * 10 + power_supply[p2];
            if tmp > max {
                max = tmp;
            }
            p2 += 1;
        }
        p1 += 1;
    }

    max
}

fn ultra_joltage(power_supply: &[usize], digits_left: usize) -> usize {
    if power_supply.len() < digits_left {
        return 0;
    }
    if digits_left == 1 {
        return *power_supply.iter().max().unwrap_or(&0);
    }

    let mut radix = 1;
    let mut max = 0;
    let mut max_index = 0;

    for _ in 1..digits_left {
        radix *= 10;
    }
    for (i, &v) in power_supply[..=power_supply.len() - digits_left].iter().enumerate() {
        if v > max {
            max = v;
            max_index = i;
        }
    }
    
    max * radix + ultra_joltage(&power_supply[max_index + 1..], digits_left - 1)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn input_works() {
        let input = "12345\n98765\n1337";
        let expected: Vec<Vec<usize>> =
            vec![vec![1, 2, 3, 4, 5], vec![9, 8, 7, 6, 5], vec![1, 3, 3, 7]];

        let input_result = read_input(input);
        assert_eq!(expected, input_result);
    }

    #[test]
    fn test_joltage_calcs() {
        let input =
            read_input("987654321111111\n811111111111119\n234234234234278\n818181911112111");
        let expected = vec![98, 89, 78, 92];
        let mut actual = vec![];

        for i in &input {
            actual.push(max_joltage(i));
        }

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_ultra_joltage() {
        let input =
            read_input("987654321111111\n811111111111119\n234234234234278\n818181911112111");
        let expected = vec![987654321111, 811111111119, 434234234278, 888911112111];
        let mut actual = Vec::new();

        for i in &input {
            actual.push(ultra_joltage(&i, 12));
        }

        assert_eq!(expected, actual)
    }
}
