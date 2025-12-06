// keeps the compiler quiet - swap between the fns in main()
#![allow(dead_code)]

use std::fs;
use std::sync::OnceLock;

// cool way to enforce singleton-like behavior
// ideal for long computation tasks like this
static DIGIT_PERMS_NEW: OnceLock<Vec<i64>> = OnceLock::new();
static DIGIT_PERMS: OnceLock<Vec<i64>> = OnceLock::new();

fn main() {
    let input = fs::read_to_string("input.txt").expect("error reading file!");
    let data = read_input(&input);
    let result = sum_invalid_new(&data);

    // let result = sum_invalid_new(&vec![(11i64, 22i64)]);

    println!("sum of all invalid IDs: {result}");
    // println!("{:?}", digit_permutations_new());
}

// this is so nice vs writing out all the boilerplate
fn read_input(input: &str) -> Vec<(i64, i64)> {
    input
        .split(',')
        .map(|s| {
            let mut range_chars = s.split('-');
            let min: i64 = range_chars
                .next()
                .expect("invalid string!")
                .parse()
                .expect("can't convert str to int");
            let max: i64 = range_chars
                .next()
                .expect("invalid string!")
                .parse()
                .expect("can't convert str to int");
            (min, max)
        })
        .collect::<Vec<(i64, i64)>>()
}

// uses a brute-force, inefficient algorithm
fn sum_invalid(data: &Vec<(i64, i64)>) -> i64 {
    let mut total: i64 = 0;
    let perms = digit_permutations();
    'outer: for (min, max) in data {
        let mut i = 0;
        while perms.get(i).expect("?") < min {
            i += 1;
            if perms.get(i).expect("?") > max {
                continue 'outer;
            }
        }
        let mut semi_total = *perms.get(i).expect("?");
        while perms.get(i + 1).expect("") <= max {
            i += 1;
            semi_total += perms.get(i).expect("?");
        }
        total += semi_total;
    }

    total
}

// same algo, it was late at night and I didn't expect it to work first try
fn sum_invalid_new(data: &Vec<(i64, i64)>) -> i64 {
    let mut total: i64 = 0;
    let perms = digit_permutations_new();
    'outer: for (min, max) in data {
        let mut i = 0;
        while perms.get(i).expect("?") < min {
            i += 1;
            if perms.get(i).expect("?") > max {
                continue 'outer;
            }
        }
        let mut semi_total = *perms.get(i).expect("?");
        while perms.get(i + 1).expect("") <= max {
            i += 1;
            semi_total += perms.get(i).expect("?");
        }
        total += semi_total;
    }

    total
}

// more cursed code to generate all perms
// the data is small enough to brute force for now
fn digit_permutations_new() -> &'static Vec<i64> {
    DIGIT_PERMS_NEW.get_or_init(|| {
        let mut perms: Vec<i64> = Vec::new();
        // repeat 1s
        for i in 2..=10 {
            for j in 1..=9 {
                perms.push(j.to_string().repeat(i as usize).parse().expect("?"));
            }
        }
        // repeat 2s
        for i in 2..=5 {
            for j in 1..=9 {
                for k in 0..=9 {
                    perms.push(format!("{j}{k}").repeat(i as usize).parse().expect("?"));
                }
            }
        }
        // repeat 3s
        for i in 2..=3 {
            for j in 1..=9 {
                for k in 0..=9 {
                    for l in 0..=9 {
                        perms.push(format!("{j}{k}{l}").repeat(i as usize).parse().expect("?"));
                    }
                }
            }
        }
        // repeat 4s
        for i in 1..=9 {
            for j in 0..=9 {
                for k in 0..=9 {
                    for l in 0..=9 {
                        perms.push(format!("{i}{j}{k}{l}").repeat(2).parse().expect("?"));
                    }
                }
            }
        }
        // repeat 5s
        for i in 1..=9 {
            for j in 0..=9 {
                for k in 0..=9 {
                    for l in 0..=9 {
                        for m in 0..=9 {
                            perms.push(format!("{i}{j}{k}{l}{m}").repeat(2).parse().expect("?"));
                        }
                    }
                }
            }
        }
        perms.sort_unstable();
        perms.dedup(); // we do overcount - so we deduplicate
        perms
    })
}

fn digit_permutations() -> &'static Vec<i64> {
    DIGIT_PERMS.get_or_init(|| {
        let mut perms = Vec::new();

        // rep 1
        for i in 1..=9 {
            perms.push(i.to_string().repeat(2).parse().expect("?"));
        }
        // rep 2
        for i in 1..=9 {
            for j in 0..=9 {
                perms.push(format!("{i}{j}").repeat(2).parse().expect("?"));
            }
        }
        // rep 3
        for i in 1..=9 {
            for j in 0..=9 {
                for k in 0..=9 {
                    perms.push(format!("{i}{j}{k}").repeat(2).parse().expect("?"));
                }
            }
        }
        // rep 4
        for i in 1..=9 {
            for j in 0..=9 {
                for k in 0..=9 {
                    for l in 0..=9 {
                        perms.push(format!("{i}{j}{k}{l}").repeat(2).parse().expect("?"));
                    }
                }
            }
        }
        // rep 5
        for i in 1..=9 {
            for j in 0..=9 {
                for k in 0..=9 {
                    for l in 0..=9 {
                        for m in 0..=9 {
                            perms.push(format!("{i}{j}{k}{l}{m}").repeat(2).parse().expect("?"));
                        }
                    }
                }
            }
        }

        perms.sort_unstable();
        perms
    })
}

// the aoc case & the first case were written by me
// the rest are ai generated clones
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn aoc_test_case() {
        let test_case = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let expected_vec = vec![
            (11, 22),
            (95, 115),
            (998, 1012),
            (1_188_511_880, 1_188_511_890),
            (222_220, 222_224),
            (1_698_522, 1_698_528),
            (446_443, 446_449),
            (38_593_856, 38_593_862),
            (565_653, 565_659),
            (824_824_821, 824_824_827),
            (2_121_212_118, 2_121_212_124),
        ];

        assert_eq!(expected_vec, read_input(&test_case));
        assert_eq!(1227775554i64, sum_invalid(&expected_vec));
    }

    #[test]
    fn test_11_22() {
        let test_data = vec![(11, 22)];
        assert_eq!(33, sum_invalid(&test_data));
    }

    #[test]
    fn test_95_115() {
        let test_data = vec![(95, 115)];
        assert_eq!(99, sum_invalid(&test_data));
    }

    #[test]
    fn test_998_1012() {
        let test_data = vec![(998, 1012)];
        assert_eq!(1010, sum_invalid(&test_data));
    }

    #[test]
    fn test_1188511880_1188511890() {
        let test_data = vec![(1_188_511_880, 1_188_511_890)];
        assert_eq!(1_188_511_885, sum_invalid(&test_data));
    }

    #[test]
    fn test_222220_222224() {
        let test_data = vec![(222_220, 222_224)];
        assert_eq!(222_222, sum_invalid(&test_data));
    }

    #[test]
    fn test_1698522_1698528() {
        let test_data = vec![(1_698_522, 1_698_528)];
        assert_eq!(0, sum_invalid(&test_data));
    }

    #[test]
    fn test_446443_446449() {
        let test_data = vec![(446_443, 446_449)];
        assert_eq!(446_446, sum_invalid(&test_data));
    }

    #[test]
    fn test_38593856_38593862() {
        let test_data = vec![(38_593_856, 38_593_862)];
        assert_eq!(38_593_859, sum_invalid(&test_data));
    }

    #[test]
    fn test_565653_565659() {
        let test_data = vec![(565_653, 565_659)];
        assert_eq!(0, sum_invalid(&test_data));
    }

    #[test]
    fn test_824824821_824824827() {
        let test_data = vec![(824_824_821, 824_824_827)];
        assert_eq!(0, sum_invalid(&test_data));
    }

    #[test]
    fn test_2121212118_2121212124() {
        let test_data = vec![(2_121_212_118, 2_121_212_124)];
        assert_eq!(0, sum_invalid(&test_data));
    }

    fn aoc_test_case_new() {
        let test_case = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let expected_vec = vec![
            (11, 22),
            (95, 115),
            (998, 1012),
            (1_188_511_880, 1_188_511_890),
            (222_220, 222_224),
            (1_698_522, 1_698_528),
            (446_443, 446_449),
            (38_593_856, 38_593_862),
            (565_653, 565_659),
            (824_824_821, 824_824_827),
            (2_121_212_118, 2_121_212_124),
        ];

        assert_eq!(expected_vec, read_input(&test_case));
        assert_eq!(4174379265, sum_invalid_new(&expected_vec));
    }

    #[test]
    fn test_11_22_new() {
        let test_data = vec![(11, 22)];
        assert_eq!(33, sum_invalid_new(&test_data));
    }

    #[test]
    fn test_95_115_new() {
        let test_data = vec![(95, 115)];
        assert_eq!(210, sum_invalid_new(&test_data));
    }

    #[test]
    fn test_998_1012_new() {
        let test_data = vec![(998, 1012)];
        assert_eq!(2009, sum_invalid_new(&test_data));
    }

    #[test]
    fn test_1188511880_1188511890_new() {
        let test_data = vec![(1_188_511_880, 1_188_511_890)];
        assert_eq!(1_188_511_885, sum_invalid_new(&test_data));
    }

    #[test]
    fn test_222220_222224_new() {
        let test_data = vec![(222_220, 222_224)];
        assert_eq!(222_222, sum_invalid_new(&test_data));
    }

    #[test]
    fn test_1698522_1698528_new() {
        let test_data = vec![(1_698_522, 1_698_528)];
        assert_eq!(0, sum_invalid_new(&test_data));
    }

    #[test]
    fn test_446443_446449_new() {
        let test_data = vec![(446_443, 446_449)];
        assert_eq!(446_446, sum_invalid_new(&test_data));
    }

    #[test]
    fn test_38593856_38593862_new() {
        let test_data = vec![(38_593_856, 38_593_862)];
        assert_eq!(38_593_859, sum_invalid_new(&test_data));
    }

    #[test]
    fn test_565653_565659_new() {
        let test_data = vec![(565_653, 565_659)];
        assert_eq!(565656, sum_invalid_new(&test_data));
    }

    #[test]
    fn test_824824821_824824827_new() {
        let test_data = vec![(824_824_821, 824_824_827)];
        assert_eq!(824824824, sum_invalid_new(&test_data));
    }

    #[test]
    fn test_2121212118_2121212124_new() {
        let test_data = vec![(2_121_212_118, 2_121_212_124)];
        assert_eq!(2121212121, sum_invalid_new(&test_data));
    }
}
