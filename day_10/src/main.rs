use std::{
    time::Instant,
    fs,
    collections::BTreeMap
};

fn part1() {
    let input = fs::read_to_string("input").unwrap();

    let mut start_pos = (-1, -1);
    let mut pipe_map = BTreeMap::new();
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
    let mut pipe_map = BTreeMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            let x = i as i32;
            let y = j as i32;
            if char == 'S' { start_pos = (x, y); }
            let neighbours = match char {
                '|' => ('|', (x-1, y), (x+1, y)),
                '-' => ('-', (x, y-1), (x, y+1)),
                'L' => ('L', (x-1, y), (x, y+1)),
                'J' => ('J', (x-1, y), (x, y-1)),
                '7' => ('7', (x+1, y), (x, y-1)),
                'F' => ('F', (x+1, y), (x, y+1)),
                'S' => ('S', (x, y), (-2, -2)),                                          // Using -2, -2 here as a marker
                _ => { continue; }
            };
            pipe_map.insert((x, y), neighbours);
        }
    }

    // Find S neighbours
    let s_neighbours: Vec<(i32, i32)> = pipe_map.iter().filter(|(_, val)| {
        val.2 == start_pos || (val.1 == start_pos && val.2 != (-2, -2))
    }).map(|(k, _)| *k).collect();
    pipe_map.insert(start_pos, ('S', s_neighbours[0], s_neighbours[1]));

    // Store loop
    let mut pipe: Vec<(i32, i32)> = vec![start_pos];
    let mut r_prev = start_pos;
    let mut right = pipe_map.get(&start_pos).unwrap().1;
    while right != start_pos {
        right = *pipe_map.get(&right).map(|(_, next_r, next_l)| if r_prev == *next_r {
            r_prev = right;
            next_l
        } else {
            r_prev = right;
            next_r
        }).unwrap();

        pipe.push(right);
    }

    let max_x = pipe.iter().map(|pos| pos.0).max().unwrap();
    let max_y = pipe.iter().map(|pos| pos.1).max().unwrap();

    let mut i_count = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            if pipe.contains(&(x, y)) {
                continue;
            }

            let winding_nr: i32 = pipe[..pipe.len()].windows(2).map(|window| {
                let pipe_pos1 = window[0];
                let pipe_pos2 = window[1];

                if pipe_pos1.1 <= y {
                    if pipe_pos2.1 > y {
                        if ( (pipe_pos2.0 - pipe_pos1.0) * (y - pipe_pos1.1) - (x - pipe_pos1.0) * (pipe_pos2.1 - pipe_pos1.1) ) >= 0 {
                            return 1;
                        }
                    }
                } else {
                    if pipe_pos2.1 <= y {
                        if ( (pipe_pos2.0 - pipe_pos1.0) * (y - pipe_pos1.1) - (x - pipe_pos1.0) * (pipe_pos2.1 - pipe_pos1.1) ) < 0 {
                            return -1;
                        }
                    }
                }
                
                return 0;
            }).sum();

            if winding_nr.abs() % 2 == 1 {
                i_count += 1;
            }
        }
    }

    println!("Found points inside the pipe: {:?}", i_count);
}

fn main() {
    let start = Instant::now();

    //part1();
    part2();

    println!("Elapsed: {:?}", start.elapsed());
}
