use std::fs;
use std::collections::HashMap;
use num::integer::lcm;


fn part1(directions: &str, map: &HashMap<String, (String, String)>) -> u32 {
    let mut steps = 0;
    let mut pos = "AAA";

    while pos != "ZZZ" {
        for dir in directions.chars() {
            let entry = map.get(pos).unwrap();
            pos = if dir == 'L' { &entry.0 } else { &entry.1 };
            steps += 1;
        }
    }
    steps
}

fn find_steps(start: &str, directions: &str, map: &HashMap<String, (String, String)>) -> u32 {
    let mut steps = 0;
    let mut pos = start;

    while !pos.ends_with('Z') {
        for dir in directions.chars() {
            let entry = map.get(pos).unwrap();
            pos = if dir == 'L' { &entry.0 } else { &entry.1 };
            steps += 1;
        }
    }
    steps
}

fn part2(directions: &str, map: &HashMap<String, (String, String)>) -> u64 {
    let steps: Vec<u64> = map.iter()
        .filter(|x| x.0.ends_with('A'))
        .map(|x| x.0.clone())
        .map(|x| find_steps(&x, directions, map) as u64)
        .collect();
    // least common multiple
    steps.iter().cloned().fold(1, lcm)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file not found");
    let mut lines = input.lines();
    let directions = lines.next().unwrap();

    let map: HashMap<String, (String, String)> = lines.skip(1)
        .map(|line| {
            let filtered: String = line.chars()
                .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                .collect();
            let items: Vec<_> = filtered
                .split_whitespace()
                .collect();

           (items[0].to_string(), (items[1].to_string(), items[2].to_string()))
        })
        .collect();

    println!("Part1: {}", part1(&directions, &map));
    println!("Part2: {}", part2(&directions, &map));
}