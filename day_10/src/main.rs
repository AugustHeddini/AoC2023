use std::{
    time::Instant,
    fs,
    collections::HashMap
};


fn part1() {
    let input = fs::read_to_string("input").unwrap();

    let mut start_pos = (-1, -1);
    let mut pipe_map = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            let x = i as i32;
            let y = j as i32;
            if char == 'S' { start_pos = (x, y); }
            let neighbours = match char {
                '|' => ((x-1, y), (x+1, y)),
                '-' => ((x, y-1), (x, y+1)),
                'L' => ((x-1, y), (x, y+1)),
                'J' => ((x-1, y), (x, y-1)),
                '7' => ((x+1, y), (x, y-1)),
                'F' => ((x+1, y), (x, y+1)),
                'S' => ((x, y), (-2, -2)),                                          // Using -2, -2 here as a marker
                _ => { continue; }
            };
            pipe_map.insert((x, y), neighbours);
        }
    }

    // Find S neighbours
    let s_neighbours: Vec<(i32, i32)> = pipe_map.iter().filter(|(_, val)| {
        val.1 == start_pos || (val.0 == start_pos && val.1 != (-2, -2))
    }).map(|(k, _)| *k).collect();
    pipe_map.insert(start_pos, (s_neighbours[0], s_neighbours[1]));

    // Walk the loop
    let mut path_length = 1;
    let (mut r_prev, mut l_prev) = (start_pos, start_pos);
    let (mut right, mut left) = pipe_map.get(&start_pos).unwrap();
    loop {  
        if right == left {
            break;
        }

        right = *pipe_map.get(&right).map(|(next_r, next_l)| if r_prev == *next_r {
            r_prev = right;
            next_l
        } else {
            r_prev = right;
            next_r
        }).unwrap();

        left = *pipe_map.get(&left).map(|(next_r, next_l)| if l_prev == *next_r {
            l_prev = left;
            next_l
        } else {
            l_prev = left;
            next_r
        }).unwrap();

        path_length += 1;
    }

    println!("Furthest point: {}", path_length);
}

fn part2() {
    let input = fs::read_to_string("input").unwrap();

    let mut start_pos = (-1, -1);
    let mut pipe_map = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            let x = i as i32;
            let y = j as i32;
            if char == 'S' { start_pos = (x, y); }
            let neighbours = match char {
                '|' => ((x-1, y), (x+1, y)),
                '-' => ((x, y-1), (x, y+1)),
                'L' => ((x-1, y), (x, y+1)),
                'J' => ((x-1, y), (x, y-1)),
                '7' => ((x+1, y), (x, y-1)),
                'F' => ((x+1, y), (x, y+1)),
                'S' => ((x, y), (-2, -2)),                                          // Using -2, -2 here as a marker
                _ => { continue; }
            };
            pipe_map.insert((x, y), neighbours);
        }
    }

    // Find S neighbours
    let s_neighbours: Vec<(i32, i32)> = pipe_map.iter().filter(|(_, val)| {
        val.1 == start_pos || (val.0 == start_pos && val.1 != (-2, -2))
    }).map(|(k, _)| *k).collect();
    pipe_map.insert(start_pos, (s_neighbours[0], s_neighbours[1]));

    // Walk the loop
    let mut path_length = 1;
    let (mut r_prev, mut l_prev) = (start_pos, start_pos);
    let (mut right, mut left) = pipe_map.get(&start_pos).unwrap();
    loop {  
        if right == left {
            break;
        }

        right = *pipe_map.get(&right).map(|(next_r, next_l)| if r_prev == *next_r {
            r_prev = right;
            next_l
        } else {
            r_prev = right;
            next_r
        }).unwrap();

        left = *pipe_map.get(&left).map(|(next_r, next_l)| if l_prev == *next_r {
            l_prev = left;
            next_l
        } else {
            l_prev = left;
            next_r
        }).unwrap();

        path_length += 1;
    }

    println!("Furthest point: {}", path_length);
}

fn main() {
    let start = Instant::now();

    //part1();
    part2();

    println!("Elapsed: {:?}", start.elapsed());
}
