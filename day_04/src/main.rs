use std::{collections::VecDeque, fs, time::Instant};

fn part1() {
    let input = fs::read_to_string("input").unwrap();

    let mut total = 0;
    for line in input.lines() {
        let mut halves = line.split(":").last().unwrap().split(" | ");

        let gotten = halves.next().unwrap().split_whitespace();
        let winning = halves
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();
        let card_score = gotten.filter(|get| winning.contains(get)).count() as u32;

        if card_score > 0 {
            total += 1 << (card_score - 1);
        }
    }

    println!("Total: {}", total);
}

fn part2() {
    let input = fs::read_to_string("input").unwrap();

    let mut total = 0;
    let mut multipliers = VecDeque::new();
    for line in input.lines() {
        let copies = multipliers.pop_front().unwrap_or(1);
        let mut halves = line.split(":").last().unwrap().split(" | ");

        let gotten = halves.next().unwrap().split_whitespace();
        let winning = halves
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();
        let card_score = gotten.filter(|get| winning.contains(get)).count();

        total += copies;
        for i in 0..card_score {
            if multipliers
                .get_mut(i)
                .and_then(|val| Some(*val += copies))
                .is_none()
            {
                multipliers.push_back(1 + copies);
            };
        }
    }

    println!("Total: {}", total);
}

fn main() {
    let start = Instant::now();

    // part1();
    part2();

    println!("Elapsed: {:?}", start.elapsed());
}
