use std::{fs, time::Instant};

fn find_next_value(sequence: Vec<i32>) -> i32 {
    if sequence.iter().all(|val| *val == 0) {
        return 0;
    }

    return sequence.last().unwrap() + find_next_value(differences(&sequence));
}

fn find_prev_value(sequence: Vec<i32>) -> i32 {
    if sequence.iter().all(|val| *val == 0) {
        return 0;
    }

    return sequence.first().unwrap() - find_prev_value(differences(&sequence));
}

fn differences(sequence: &Vec<i32>) -> Vec<i32> {
    return sequence
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();
}

fn part1() {
    let input = fs::read_to_string("input").unwrap();

    let res: i32 = input
        .lines()
        .map(|line| {
            find_next_value(
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
            )
        })
        .sum();

    println!("Result: {}", res);
}

fn part2() {
    let input = fs::read_to_string("input").unwrap();

    let res: i32 = input
        .lines()
        .map(|line| {
            find_prev_value(
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
            )
        })
        .sum();

    println!("Result: {}", res);
}

fn main() {
    let start = Instant::now();

    // part1();
    part2();

    println!("Elapsed: {:?}", start.elapsed());
}
