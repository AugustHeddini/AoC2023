use std::{
    time::Instant,
    fs
};

fn distance(gal1: &(usize, usize), gal2: &(usize, usize), exp_x: &Vec<bool>, exp_y: &Vec<bool>) -> i64 {
    let x_dist = gal1.0.abs_diff(gal2.0) as i64 + (999999 * exp_x[gal1.0.min(gal2.0)..gal2.0.max(gal1.0)].iter().filter(|x| **x).count() as i64);
    let y_dist = gal1.1.abs_diff(gal2.1) as i64 + (999999 * exp_y[gal1.1.min(gal2.1)..gal2.1.max(gal1.1)].iter().filter(|y| **y).count() as i64);
    return x_dist + y_dist;
}

fn main() {
    let start = Instant::now();

    let input = fs::read_to_string("input").unwrap();
    let cols = input.lines().last().unwrap().len();
    let rows = input.lines().count();

    let mut starmap = vec![];
    let mut expanded_x = vec![false; cols];
    let mut expanded_y = vec![false; rows];

    // Build the starmap and find expanded rows and columns
    for (i, line) in input.lines().enumerate() {
        let char_vec: Vec<char> = line.chars().collect();
        if char_vec.iter().all(|chr| *chr == '.') {
            expanded_x[i] = true;
        }
        starmap.push(char_vec);
    }
    (0..cols).enumerate().for_each(|(j, col)| {
        if (0..rows).map(|row| {
            starmap[row][col]
        }).all(|chr| chr == '.') {
            expanded_y[j] = true;
        }
    });

    // Find all the galaxies
    let mut galaxies = vec![];
    starmap.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, chr)| {
            if *chr == '#' {
                galaxies.push((i, j));
            }
        })
    });

    // Sum the distances
    let mut distance_sum: i64 = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            distance_sum += distance(&galaxies[i], &galaxies[j], &expanded_x, &expanded_y);
        }
    }
    
    println!("Total sum of distances: {}", distance_sum);

    println!("Elapsed: {:?}", start.elapsed());
}
