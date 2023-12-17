use std::{collections::HashMap, fs, time::Instant};

fn transform_to_direction<T>(dir: char) -> fn(&(T, T)) -> &T {
    match dir {
        'L' => |tuple| &tuple.0,
        'R' => |tuple| &tuple.1,
        _ => panic!("Invalid direction char!"),
    }
}

fn part1() {
    let input = fs::read_to_string("input").unwrap();
    let mut lines = input.lines();

    let mut moves = lines
        .next()
        .unwrap()
        .chars()
        .map(|char| transform_to_direction::<&str>(char))
        .cycle();
    lines.next(); // Skip empty line after moves line
    
    let mut desert_map = HashMap::new();
    for line in lines {
        let mut keys = line
            .split(['(', ')', '=', ' ', ','])
            .filter(|val| !val.is_empty());
        desert_map.insert(
            keys.next().unwrap(),
            (keys.next().unwrap(), keys.next().unwrap()),
        );
    }

    let mut pos = "AAA";
    let mut res = 0;
    loop {
        pos = moves.next().unwrap()(desert_map.get(pos).unwrap());
        res += 1;
        if pos == "ZZZ" {
            break;
        }
    }

    println!("Total moves needed: {}", res);
}

fn vector_lcm(vec: &[i64]) -> i64 {
    if vec.len() == 1 {
        return vec[0];
    }
    let a = vec[0];
    let b = vector_lcm(&vec[1..]);
    return a * b / gcd(&a, &b);
}

fn gcd(a: &i64, b: &i64) -> i64 {
    return if *b == 0 {
        *a
    } else {
        gcd(b, &(a % b))
    }
}

fn part2() {
    let input = fs::read_to_string("input").unwrap();
    let mut lines = input.lines();

    let mut moves = lines
        .next()
        .unwrap()
        .chars()
        .map(|char| transform_to_direction::<&str>(char))
        .cycle();
    lines.next(); // Skip empty line after moves line
    
    let mut desert_map = HashMap::new();
    for line in lines {
        let mut keys = line
            .split(['(', ')', '=', ' ', ','])
            .filter(|val| !val.is_empty());
        desert_map.insert(
            keys.next().unwrap(),
            (keys.next().unwrap(), keys.next().unwrap()),
        );
    }

    let mut positions: Vec<&&str> = desert_map.keys().filter(|key| key.ends_with('A')).collect();
    let mut results = vec![0; positions.len()];
    while !positions.is_empty() {
        let curr_move = moves.next().unwrap();
        for (i, pos) in positions.iter_mut().enumerate() {
            *pos = curr_move(desert_map.get(*pos).unwrap());
            results[i] += 1;
        }
        positions = positions.iter().filter(|pos| !pos.ends_with('Z')).map(|val| *val).collect();
    }
    let res = vector_lcm(&results);

    println!("Total moves needed: {}", res);
}

fn main() {
    let start = Instant::now();

    //part1();
    part2();

    println!("Elapsed: {:?}", start.elapsed());
}
