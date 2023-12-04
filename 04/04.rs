use std::fs;

fn part1(matches: &Vec<u32>) -> u32 {
    matches.iter().map(|x| (1 << x)/2).sum()
}

fn part2(matches: &Vec<u32>) -> u32 {
    let mut cards: Vec<u32> = vec![1; matches.len()];

    for (i, &count) in matches.iter().enumerate() {
        for j in 1..=count as usize {
            if i + j < cards.len() {
                cards[i + j] += cards[i];
            }
        }
    }
    cards.iter().sum()
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Input file not found");

    let matches: Vec<u32> = data.lines().map(|line| {
        let numbers: Vec<Vec<u32>> = line.split(":").last()
            .unwrap()
            .split(" | ")
            .map(|nums| nums.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
            ).collect();

        let winning = numbers.first().unwrap();
        let drawn = numbers.last().unwrap();
        drawn.iter()
            .filter(|num| winning.contains(&num))
            .count() as u32
    }).collect();

    println!("Part1: {}", part1(&matches));
    println!("Part2: {}", part2(&matches));
}