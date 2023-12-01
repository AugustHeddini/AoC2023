use std::{collections::HashMap, fs, time::Instant};

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input").unwrap();

    let number_map = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let cal_val: u16 = input
        .lines()
        .map(|line| {
            let mut first = (999, ' ');
            let mut last = (0, ' ');

            number_map.iter().for_each(|number| {
                line.match_indices(number.0).for_each(|idx| {
                    if idx.0 < first.0 {
                        first = (idx.0, *number.1)
                    }
                    if idx.0 > last.0 {
                        last = (idx.0, *number.1)
                    }
                });
            });
            let mut string_line = line.to_string();
            if first.1 != ' ' {
                string_line.insert(first.0, first.1)
            };
            if last.1 != ' ' {
                string_line.insert(last.0 + 1, last.1)
            };

            let mut chars = string_line.chars();
            let first: char = chars.find(|c| c.is_digit(10)).unwrap();
            let last: char = chars
                .rev()
                .find(|c| c.is_digit(10))
                .unwrap_or_else(|| first);
            return format!("{}{}", first, last).parse::<u16>().unwrap();
        }).sum::<u16>();

    let runtime = start.elapsed();

    println!("Sum: {:?}", cal_val);
    println!("Time {:.2?}", runtime);
}
