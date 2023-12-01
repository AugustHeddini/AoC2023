use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let cal_val: Vec<u16> = input.lines().map(|line| {
        let mut chars = line.chars();
        let first: char = chars.find(|c| c.is_digit(10)).unwrap();
        let last: char = chars.rev().find(|c| c.is_digit(10)).unwrap_or_else(|| first);
        format!("{}{}", first, last).parse::<u16>().unwrap()
    }).collect();

    println!("Values: {:?}", cal_val);
    println!("Sum: {:?}", cal_val.iter().sum::<u16>());
}
