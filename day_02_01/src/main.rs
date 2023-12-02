use std::fs;

const RED: i8 = 12;
const GREEN: i8 = 13;
const BLUE: i8 = 14;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut total = 0;
    for (i, line) in input.lines().enumerate() {
        if !line.split([':', ';']).skip(1).any(|split| {
            split
                .split(',')
                .map(|draw| {
                    let pairs: Vec<&str> = draw.split_whitespace().collect();
                    match *pairs.last().unwrap() {
                        "red" => pairs.first().unwrap().parse::<i8>().unwrap() > RED,
                        "green" => pairs.first().unwrap().parse::<i8>().unwrap() > GREEN,
                        "blue" => pairs.first().unwrap().parse::<i8>().unwrap() > BLUE,
                        _ => false,
                    }
                })
                .any(|impossible| impossible)
        }) {
            total += i + 1;
        }
    }

    println!("Total: {}", total);
}
