use std::cmp;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("missing file");
    let (mut ranges, ingredients) = read_input(&input);
    let fresh_count = count_fresh(&ranges, &ingredients);

    println!("found {fresh_count} fresh ingredient IDs");

    let total_fresh = count_all_fresh(&mut ranges);
    /* 
    attempts:
    incorrect! 422046943237045 (too high)
    incorrect! 421394193638096 (too high)

    there was a bug with new_range != ranges[counter] that over counted
    fixed now and works!
     */
    println!("found a total of {total_fresh} fresh ingredient IDs");
}

fn count_all_fresh(ranges: &mut Vec<(usize, usize)>) -> usize {
    // tbh sorting might be slower but this swaps everything in place vs copying over
    ranges.sort_unstable_by_key(|&(min, _)| min);

    // realized the merges could back-propagate so kept looping until unchanged
    // could rewrite the whole algorithm to be balanced around ranges[1]
    // then check left/right w/ error bounds & merge both sides but
    // this band-aid solution fixes it in ~10s of dev time
    loop {
        let initial = ranges.len();

        let mut counter = 0;
        while counter < ranges.len() - 1 {
            let curr = &ranges[counter];
            let next = &ranges[counter + 1];
            if (curr.0 <= next.1 && next.1 <= curr.1) || (curr.0 <= next.0 && next.0 <= curr.1) {
                let new_range = extend_range(&ranges[counter], &ranges[counter + 1]);

                if new_range != ranges[counter] {
                    ranges[counter] = new_range;
                }
                // and on top of that these removes are expensive but rust is fast
                ranges.remove(counter + 1);
            } else {
                counter += 1;
            }
        }

        if initial == ranges.len() {
            break;
        }
    }

    ranges.iter().map(|&(min, max)| max - min + 1).sum()
}

fn extend_range(range1: &(usize, usize), range2: &(usize, usize)) -> (usize, usize) {
    let new_min = cmp::min(range1.0, range2.0);
    let new_max = cmp::max(range1.1, range2.1);

    (new_min, new_max)
}

fn count_fresh(ranges: &Vec<(usize, usize)>, ingredients: &Vec<usize>) -> usize {
    let mut total = 0;

    'outer: for ingredient in ingredients {
        for (min, max) in ranges {
            if min <= ingredient && ingredient <= max {
                total += 1;
                continue 'outer;
            }
        }
    }

    total
}

// could've used [usize; 2] or a struct but this doesn't need to scale
fn read_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    for line in input.lines() {
        if line.contains("-") {
            let values = line
                .split("-")
                .map(|v| v.parse::<usize>().expect("can't parse range to usize"))
                .collect::<Vec<usize>>();
            if values.len() != 2 {
                panic!("invalid range!")
            }
            ranges.push((values[0], values[1]));
        } else if line.len() > 0 { // skips empty line break
            ingredients.push(line.parse().expect("can't parse ingredient to usize"));
        }
    }

    (ranges, ingredients)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_read_ingredient_ranges() {
        let sample_input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let expected_ranges = vec![
            (3, 5),
            (10, 14),
            (16, 20),
            (12, 18),
        ];
        let expected_ingredients = vec![1, 5, 8, 11, 17, 32];

        let (ranges, ingredients) = read_input(sample_input);
        
        assert_eq!(expected_ranges, ranges);
        assert_eq!(expected_ingredients, ingredients);
    }

    #[test]
    fn test_example_part_one() {
        let sample_input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let (ranges, ingredients) = read_input(sample_input);

        assert_eq!(count_fresh(&ranges, &ingredients), 3);
    }

    #[test]
    fn test_example_part_two() {
        let sample_input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let (mut ranges, _) = read_input(sample_input);
        let expected_ranges = vec![
            (3, 5),
            (10, 20),
        ];

        let result = count_all_fresh(&mut ranges);

        // it was convenient to test this w/ mutations so I wrote it this way
        assert_eq!(expected_ranges, ranges);
        assert_eq!(result, 14);
    }

}