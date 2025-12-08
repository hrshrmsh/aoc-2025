use std::fs;

#[derive(PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
struct JunctionBox {
    // id serves no real purpose - it was some boilerplate I thought I'd need for later
    // I wound up not using it, but it's left here as an artifact
    id: usize,
    coors: Vec<usize>,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("error reading file");
    let junctions = read_input(&input);
    let (r1, _) = circuit_statistics(&junctions, 1000);
    let (_, r2) = circuit_statistics(&junctions, 0);

    // both worked first try today
    println!("circuit multiplication result: {r1}");
    println!("last two x multiplication result: {r2}");
}

// decided to do both in one function since it "seemed" to be a quick and simple change
// spoiler: it was not - I don't recommend it because it enforces weird design decisions
fn circuit_statistics(junctions: &Vec<JunctionBox>, limit: usize) -> (usize, usize) {
    let mut distances = vec![vec![0.0; junctions.len()]; junctions.len()];
    let mut connections = vec![];
    let mut circuits: Vec<Vec<usize>> = vec![];
    let mut last_two_x_product = 0;

    // anyways to roughly explain the algorithm, just make a 2d cache of all distances
    // as well as another vec of all possible permutations of connections
    for i in 0..junctions.len() {
        for j in i + 1..junctions.len() {
            distances[i][j] = distance(&junctions[i], &junctions[j]);
            connections.push((i, j));
        }
    }

    // sort the connections based on cached distances
    connections.sort_unstable_by(|&a, &b| distances[a.0][a.1].total_cmp(&distances[b.0][b.1]));
    if limit != 0 {
        connections.truncate(limit);
    }

    // then use each connection to create a graph
    // I used a vec<vec<usize>> for my graph b/c it's the tool I'm the most comfortable with
    'outer: for (i, j) in connections {
        let mut i_pos = 0;
        let mut j_pos = 0;
        let mut found_i = false;
        let mut found_j = false;

        // first try to find i & j in our graph
        for (pos, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&i) {
                i_pos = pos;
                found_i = true;
            }
            if circuit.contains(&j) {
                j_pos = pos;
                found_j = true;
            }
        }

        // then merge / create / append subgraphs as needed
        match (found_i, found_j) {
            (false, false) => circuits.push(vec![i, j]),
            (true, false) => circuits[i_pos].push(j),
            (false, true) => circuits[j_pos].push(i),
            (true, true) => {
                if j_pos != i_pos {
                    let buffer = circuits[j_pos].clone();
                    circuits[i_pos].extend(buffer);
                    circuits.swap_remove(j_pos);
                }
            }
        }

        // it turns out to be mildly annoying to figure out when the last circuit was joined
        // so I elected for a brute force check each connection instead of being smart
        for circuit in &circuits {
            if circuit.len() == junctions.len() {
                last_two_x_product = junctions[i].coors[0] * junctions[j].coors[0];
                break 'outer;
            }
        }
    }

    // top three calculation was straightforward enough
    circuits.sort_unstable_by_key(|c| c.len());
    circuits.reverse();
    let top_three_product = circuits.get(0).unwrap_or(&vec![]).len()
        * circuits.get(1).unwrap_or(&vec![]).len()
        * circuits.get(2).unwrap_or(&vec![]).len();

    // et voila problem solved
    (top_three_product, last_two_x_product)
}

fn distance(j1: &JunctionBox, j2: &JunctionBox) -> f64 {
    let dx = j1.coors[0].abs_diff(j2.coors[0]);
    let dy = j1.coors[1].abs_diff(j2.coors[1]);
    let dz = j1.coors[2].abs_diff(j2.coors[2]);

    ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
}

fn read_input(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .enumerate()
        .map(|(id, s)| JunctionBox {
            id,
            coors: s
                .trim()
                .split(",")
                .map(|v| v.parse().expect("invalid integer in input"))
                .collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_input() {
        let input = "162,817,812\n57,618,57\n906,360,560";
        let expected_junctions = vec![
            JunctionBox {
                id: 0,
                coors: vec![162, 817, 812],
            },
            JunctionBox {
                id: 1,
                coors: vec![57, 618, 57],
            },
            JunctionBox {
                id: 2,
                coors: vec![906, 360, 560],
            },
        ];

        let junctions = read_input(input);
        assert_eq!(junctions, expected_junctions);
    }

    #[test]
    fn test_sample_part_one() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";
        let junctions = read_input(input);
        let (result, _) = circuit_statistics(&junctions, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_sample_part_two() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";
        let junctions = read_input(input);
        let (_, result) = circuit_statistics(&junctions, 0);
        assert_eq!(result, 25272);
    }
}
