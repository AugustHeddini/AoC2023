use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut total = 0;
    for line in input.lines() {
        let mut min_reds = 0;
        let mut min_greens = 0;
        let mut min_blues = 0;

        line.split([':', ';']).skip(1).for_each(|split| {
            split.split(',').for_each(|draw| {
                let pairs: Vec<&str> = draw.split_whitespace().collect();
                let number_cubes = pairs.first().unwrap().parse::<i8>().unwrap();
                match *pairs.last().unwrap() {
                    "red" => min_reds = i32::max(min_reds, number_cubes as i32),
                    "green" => min_greens = i32::max(min_greens, number_cubes as i32),
                    "blue" => min_blues = i32::max(min_blues, number_cubes as i32),
                    _ => (),
                }
            })
        });

        total += min_reds * min_greens * min_blues;
    }

    println!("Total: {}", total);
}
