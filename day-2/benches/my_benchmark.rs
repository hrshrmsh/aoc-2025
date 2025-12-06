use std::fs;

use criterion::{Criterion, criterion_group, criterion_main};

fn sum_invalid(data: &Vec<(i64, i64)>, perms: &Vec<i64>) -> i64 {
    let mut total: i64 = 0;
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

fn criterion_benchmark(c: &mut Criterion) {
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
    perms.dedup();

    let input = fs::read_to_string("input.txt")
        .expect("error reading file!")
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
        .collect::<Vec<(i64, i64)>>();

    c.bench_function("sum_invalid", |b| b.iter(|| sum_invalid(&input, &perms)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
