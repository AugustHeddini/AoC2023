use std::{fs, time::Instant};

fn part1() {
    let input = fs::read_to_string("input").unwrap();
    let mut lines = input.lines();

    let times: Vec<i32> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|time| time.parse().unwrap())
        .collect();
    let distances: Vec<i32> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|dist| dist.parse().unwrap())
        .collect();

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
    let time: i64 = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();
    let dist: i64 = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();

    let res = (1..time).filter(|push| push * (time - push) > dist).count();

    println!("Res: {}", res);
}

fn main() {
    let start = Instant::now();

    //part1();
    part2();

    println!("Elapsed: {:?}", start.elapsed());
}
