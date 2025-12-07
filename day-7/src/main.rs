use std::fs;

// with yesterday's mess, this time I proactively used a struct to organize the data
#[derive(PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
struct TachyonManifold {
    start: (usize, usize),
    splitters: Vec<(usize, usize)>,
    len: usize,
}

fn main() {
    let input_text = fs::read_to_string("input.txt").expect("error reading file");
    let manifold = read_input(&input_text);
    let beam_total = count_beams(&manifold);
    let timelines = count_timelines(&manifold);

    // today's solutions both worked first time
    println!("total beams: {beam_total}");
    println!("total timelines: {timelines}");
}

// I thought it was easier to represent everything as a bunch of coordinates
// then you just update a position array and swap it with the buffer each time y-level changes
fn count_beams(manifold: &TachyonManifold) -> usize {
    let mut beam_split_count = 0;
    let mut active_beams = vec![0usize; manifold.len];
    active_beams[manifold.start.1] = 1;
    let mut new_beams: Vec<usize> = active_beams.clone();
    let mut prev_y = 0;

    for (splitter_y, splitter_x) in &manifold.splitters {
        if *splitter_y != prev_y {
            active_beams = new_beams;
            new_beams = active_beams.clone();
            prev_y = *splitter_y;
        }
        if active_beams[*splitter_x] == 1 {
            beam_split_count += 1;
            new_beams[*splitter_x] = 0;
            if *splitter_x > 0 {
                new_beams[splitter_x - 1] = 1;
            }
            if *splitter_x < manifold.len - 1 {
                new_beams[splitter_x + 1] = 1;
            }
        }
    }

    beam_split_count
}

// exact same as above, but cascades changes instead of incrementing by 1
// the position array is now a "beam power" array conceptually
fn count_timelines(manifold: &TachyonManifold) -> usize {
    // note that there's no off by 1 error - we start at timeline 1
    let mut timelines = 1;
    let mut active_beams = vec![0usize; manifold.len];
    active_beams[manifold.start.1] = 1;
    let mut new_beams: Vec<usize> = active_beams.clone();
    let mut prev_y = 0;

    for (splitter_y, splitter_x) in &manifold.splitters {
        if *splitter_y != prev_y {
            active_beams = new_beams;
            new_beams = active_beams.clone();
            prev_y = *splitter_y;
        }
        // so now we increment by this instead of 1
        let beam_power = active_beams[*splitter_x];
        if beam_power >= 1 {
            timelines += beam_power;
            // keep in mind there's 0 timelines with beams under splitters
            new_beams[*splitter_x] = 0;
            if *splitter_x > 0 {
                new_beams[splitter_x - 1] += beam_power;
            }
            if *splitter_x < manifold.len - 1 {
                new_beams[splitter_x + 1] += beam_power;
            }
        }
    }

    timelines
}

fn read_input(input: &str) -> TachyonManifold {
    let mut start = (0, 0);
    let mut splitters = vec![];

    let mut lines = input.lines();

    let first_line = lines.next().expect("input string too small").trim();
    let len = first_line.len();

    start.1 = first_line.find("S").expect("no start position given");

    for line in lines.enumerate() {
        line.1
            .char_indices()
            .filter(|&(_, ch)| ch == '^')
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
            .into_iter()
            .for_each(|y| splitters.push((line.0 + 1, y)));
    }

    TachyonManifold {
        start,
        splitters,
        len,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_input() {
        let input_str =
            ".......S.......\n...............\n.......^.......\n...............\n......^.^......";
        let expected_manifold = TachyonManifold {
            start: (0, 7),
            splitters: vec![(2, 7), (4, 6), (4, 8)],
            len: 15,
        };

        let manifold = read_input(input_str);
        assert_eq!(manifold, expected_manifold);
    }

    #[test]
    fn sample_test_part_one() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";
        let manifold = read_input(input);

        let result = count_beams(&manifold);
        assert_eq!(result, 21);
    }

    #[test]
    fn sample_test_part_two() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";
        let manifold = read_input(input);

        let result = count_timelines(&manifold);
        assert_eq!(result, 40);
    }
}
