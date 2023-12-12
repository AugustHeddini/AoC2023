use std::{fs, time::Instant};

fn part1() {
    let input = fs::read_to_string("input").unwrap();
    let mut lines = input.lines();

    let [times, distances]: [Vec<i32>; 2] =
        [lines.next().unwrap(), lines.next().unwrap()].map(|arr| {
            arr.split(':')
                .last()
                .unwrap()
                .split_whitespace()
                .map(|nr| nr.parse().unwrap())
                .collect()
        });

    let mut res = 1;
    for (time, dist) in times.iter().zip(distances) {
        res *= {
            (1..*time)
                .filter(|push| push * (time - push) > dist)
                .count()
        };
    }

    println!("Res: {}", res);
}

fn part2() {
    let input = fs::read_to_string("input").unwrap();

    let mut lines = input.lines();

    let [time, dist] = [lines.next().unwrap(), lines.next().unwrap()].map(|nr| {
        nr.split(':')
            .last()
            .unwrap()
            .replace(" ", "")
            .parse::<i64>()
            .unwrap()
    });

    let res = (1..time).filter(|push| push * (time - push) > dist).count();

    println!("Res: {}", res);
}

fn main() {
    let start = Instant::now();

    // part1();
    part2();

    println!("Elapsed: {:?}", start.elapsed());
}
