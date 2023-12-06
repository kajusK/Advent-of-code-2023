use std::fs;

fn remap(id: u64, ranges: &Vec<(u64, u64, u64)>) -> u64 {
    ranges.iter()
        .filter(|(_, source, count)| source <= &id && source + count > id)
        .map(|(dest, source, _)| dest + (id - source))
        .next()
        .unwrap_or(id)
}

fn get_lowest(seeds: &Vec<u64>, mapping: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    seeds.iter().copied().map(|seed| {
        mapping.iter().fold(seed, |id, map| remap(id, map))
    }).min().unwrap()
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Input file not found");
    let mut parts = data.split("\n\n");

    let seeds: Vec<u64> = parts.next().unwrap().split(": ")
        .last().unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let mapping: Vec<Vec<(u64, u64, u64)>> = parts.map(|part| {
            part.lines()
            .skip(1)
            .map(|line| {
                let numbers: Vec<u64> = line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect();

                (numbers[0], numbers[1], numbers[2])
            })
            .collect()
        }).collect();

    let part1 = get_lowest(&seeds, &mapping);
    println!("{}", part1);
}