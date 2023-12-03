use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
    time::Instant,
};

fn part1() {
    let input = fs::read_to_string("input").unwrap();

    let num_regex = Regex::new(r"[0-9]+").expect("Invalid regex!");
    let mut nums: Vec<(i16, usize, (usize, usize))> = vec![];
    let mut syms = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        num_regex.find_iter(line).for_each(|res| {
            nums.push((
                res.as_str().parse().unwrap(),
                (res.end() - res.start()),
                (res.start(), y),
            ));
        });
        for (x, _) in line.match_indices(|c: char| c != '.' && !c.is_digit(10)) {
            syms.insert((x, y));
        }
    }

    let mut total = 0;
    for (number, len, (x, y)) in nums {
        // Check sides (including diagonals)
        if (x > 0
            && (syms.contains(&(x - 1, y))
                || syms.contains(&(x - 1, y + 1))
                || (y > 0 && syms.contains(&(x - 1, y - 1)))))
            || (syms.contains(&(x + len as usize, y))
                || syms.contains(&(x + len as usize, y + 1))
                || (y > 0 && syms.contains(&(x + len as usize, y - 1))))
        {
            total += number as i32;
            continue;
        }
        // Check middle
        for idx in 0..len {
            if syms.contains(&(x + idx as usize, y + 1))
                || (y > 0 && syms.contains(&(x + idx as usize, y - 1)))
            {
                total += number as i32;
                continue;
            }
        }
    }

    println!("Found total: {}", total);
}

fn part2() {
    let input = fs::read_to_string("input").unwrap();

    let num_regex = Regex::new(r"[0-9]+").expect("Invalid regex!");
    let mut nums: Vec<(i16, usize, (usize, usize))> = vec![];
    let mut gears = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        num_regex.find_iter(line).for_each(|res| {
            nums.push((
                res.as_str().parse().unwrap(),
                (res.end() - res.start()),
                (res.start(), y),
            ));
        });
        for (x, _) in line.match_indices(|c: char| c == '*') {
            gears.insert((x, y), (1i64, 0));
        }
    }

    for (number, len, (x, y)) in nums {
        // Check sides
        if x > 0 {
            if let Some(gear) = gears.get_mut(&(x - 1, y)) {
                gear.0 *= number as i64;
                gear.1 += 1;
                continue;
            }
        }
        if let Some(gear) = gears.get_mut(&(x + len as usize, y)) {
            gear.0 *= number as i64;
            gear.1 += 1;
            continue;
        }

        // Check diagonals
        if x > 0 {
            if let Some(gear) = gears.get_mut(&(x - 1, y + 1)) {
                gear.0 *= number as i64;
                gear.1 += 1;
                continue;
            }
            if y > 0 {
                if let Some(gear) = gears.get_mut(&(x - 1, y - 1)) {
                    gear.0 *= number as i64;
                    gear.1 += 1;
                    continue;
                }
            }
        }
        if let Some(gear) = gears.get_mut(&(x + len, y + 1)) {
            gear.0 *= number as i64;
            gear.1 += 1;
            continue;
        }
        if y > 0 {
            if let Some(gear) = gears.get_mut(&(x + len, y - 1)) {
                gear.0 *= number as i64;
                gear.1 += 1;
                continue;
            }
        }

        // Check middle
        for idx in x..x + len {
            if y > 0 {
                if let Some(gear) = gears.get_mut(&(idx, y - 1)) {
                    gear.0 *= number as i64;
                    gear.1 += 1;
                    continue;
                }
            }
            if let Some(gear) = gears.get_mut(&(idx as usize, y + 1)) {
                gear.0 *= number as i64;
                gear.1 += 1;
                continue;
            }
        }
    }

    println!(
        "Found total: {}",
        gears
            .values()
            .filter(|(_, accesses)| *accesses == 2)
            .fold(0, |acc, (x, _)| acc + x)
    );
}

fn main() {
    let start = Instant::now();

    // part1();
    part2();

    println!("Time Elapsed: {:?}", start.elapsed());
}
