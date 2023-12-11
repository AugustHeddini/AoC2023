use std::{collections::HashMap, fs, time::Instant};

fn part1() {
    let input = fs::read_to_string("input").unwrap();

    let mut segments = input
        .split("\n\n")
        .map(|seg| seg.split(":").last().unwrap().trim());
    let mut seeds: Vec<i64> = vec![];
    segments
        .next()
        .unwrap()
        .split_whitespace()
        .for_each(|seed| seeds.push(seed.parse().unwrap()));

    let mut maps: Vec<HashMap<i64, i64>> = vec![HashMap::new(); 7];
    let mut ranges: Vec<Vec<(i64, i64)>> = vec![];

    for (i, segment) in segments.enumerate() {
        ranges.push(vec![]);
        for line in segment.lines() {
            let mut nums = line.split_whitespace();
            let dest: i64 = nums.next().unwrap().parse().unwrap();
            let source: i64 = nums.next().unwrap().parse().unwrap();
            maps[i].insert(source, dest);

            ranges[i].push((
                source,
                source + nums.next().unwrap().parse::<i64>().unwrap(),
            ));
        }
    }

    let res = seeds
        .iter()
        .map(|seed| {
            let mut index = *seed;
            for (i, map) in maps.iter().enumerate() {
                let map_index = ranges[i]
                    .iter()
                    .find(|range| range.0 <= index && range.1 > index)
                    .unwrap_or(&(index, 0))
                    .0;
                index = map.get(&map_index).unwrap_or(&index) + (index - map_index);
            }
            index
        })
        .min()
        .unwrap();

    println!("Minimum seed location: {}", res);
}

fn part2() {
    let input = fs::read_to_string("input").unwrap();

    let mut segments = input
        .split("\n\n")
        .map(|seg| seg.split(":").last().unwrap().trim());
    let mut seeds: Vec<i64> = vec![];
    segments
        .next()
        .unwrap()
        .split_whitespace()
        .for_each(|seed| seeds.push(seed.parse().unwrap()));

    let mut maps: Vec<HashMap<i64, i64>> = vec![HashMap::new(); 7];
    let mut ranges: Vec<Vec<(i64, i64)>> = vec![];

    for (i, segment) in segments.enumerate() {
        ranges.push(vec![]);
        for line in segment.lines() {
            let mut nums = line.split_whitespace();
            let dest: i64 = nums.next().unwrap().parse().unwrap();
            let source: i64 = nums.next().unwrap().parse().unwrap();
            maps[i].insert(source, dest);

            ranges[i].push((
                source,
                source + nums.next().unwrap().parse::<i64>().unwrap(),
            ));
        }
    }

    // Sort ranges
    ranges
        .iter_mut()
        .for_each(|range_list| (*range_list).sort_by(|(a, _), (b, _)| a.cmp(b)));

    let res = seeds
        .chunks(2)
        .map(|seed_range| {
            let mut location = i64::MAX;
            let mut seed = seed_range[0];
            while seed < (seed_range[0] + seed_range[1]) {
                let mut index = seed;
                let mut jump = i64::MAX;
                for (i, map) in maps.iter().enumerate() {
                    let in_range = ranges[i]
                        .iter()
                        .find(|range| range.0 <= index && range.1 > index);
                    let map_index = in_range.unwrap_or(&(index, 0)).0;

                    // Find smallest distance to next range
                    jump = jump.min(
                        in_range
                            .unwrap_or(&(
                                0,
                                ranges[i]
                                    .iter()
                                    .find(|range| range.0 > index)
                                    .unwrap_or(&(index + 1, 0))
                                    .0,
                            ))
                            .1
                            - index,
                    );

                    index = map.get(&map_index).unwrap_or(&index) + (index - map_index);
                }
                seed += jump;
                location = location.min(index);
            }
            location
        })
        .min()
        .unwrap();

    println!("Minimum seed location: {}", res);
}

fn main() {
    let start = Instant::now();

    //part1();
    part2();

    println!("Elapsed: {:?}", start.elapsed());
}
