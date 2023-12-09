use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

fn get_normal_level(input: &str) -> u32 {
    let mut counts: HashMap<char, u32> = HashMap::new();
    for c in input.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    match counts.len() {
        // five of a kind
        1 => 7,
        // four of a kind or full house
        2 => if counts.values().any(|cnt| *cnt == 4) { 6 } else { 5 },
        // three of a kind or two pair
        3 => if counts.values().any(|cnt| *cnt == 3) { 4 } else { 3 },
        // one pair
        4 => 2,
        // anything else
        _ => 1
    }
}

fn get_joker_level(input: &str) -> u32 {
    input.chars()
        .collect::<HashSet<char>>()
        .iter()
        .map(|c| get_normal_level(&input.replace('J', &c.to_string())))
        .max().unwrap()
        .max(get_normal_level(input))
}

fn to_num(input: &str, ranks: &Vec<char>) -> u32 {
    input.chars().fold(0, |acc, c| 
        (acc * 16) + ranks.iter().position(|x| x == &c).unwrap() as u32
    )
}

fn compare_cards(a: &str, b: &str, ranks: &Vec<char>, get_level: fn(&str) -> u32) -> Ordering {
    let com_res = get_level(a).cmp(&get_level(b));
    if com_res == Ordering::Equal { to_num(a, ranks).cmp(&to_num(b, ranks)) } else { com_res }
}

fn get_result(sorted: Vec<(&str, u32)>) -> u32 {
    sorted.iter().enumerate().fold(0, |acc, (index, &value)| acc + (index as u32+1)*value.1)
}

fn main() {
    let ranks = vec!['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    let ranks_joker = vec!['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

    let input = fs::read_to_string("input.txt").expect("Input file not found");
    let mut cards: Vec<(&str, u32)> = input.lines()
        .map(|line| {
            let mut spl = line.split_whitespace();
            (spl.next().unwrap(), spl.last().unwrap().parse().unwrap())
        })
        .collect();

    cards.sort_by(|a, b| compare_cards(a.0, b.0, &ranks, get_normal_level));
    let part1 = get_result(cards.clone());

    cards.sort_by(|a, b| compare_cards(a.0, b.0, &ranks_joker, get_joker_level));
    let part2 = get_result(cards);

    println!("Part1: {part1}");
    println!("Part2: {part2}");
}