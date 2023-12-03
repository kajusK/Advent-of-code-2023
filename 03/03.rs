use std::fs;
use std::collections::HashMap;

fn is_adjacent(pos: (i32, i32), positions: &Vec<(i32, i32)>) -> bool {
    positions.iter()
        .any(|&(x, y)| (x - pos.0).abs() <= 1 && (y - pos.1).abs() <= 1)
}

fn part1(numbers: &Vec<(u32, Vec<(i32, i32)>)>, symbols: &HashMap<(i32, i32), char>) -> u32 {
numbers.iter()
        .filter(|(_, positions)| positions.iter().any(|pos| is_adjacent(*pos, &symbols.keys().cloned().collect())))
        .map(|(number, _)| number).sum()
}

fn gear_ratio(pos: (i32, i32), numbers: &Vec<(u32, Vec<(i32, i32)>)>) -> u32 {
    let gears = numbers.iter().filter(|(_, positions)| is_adjacent(pos, &positions.iter().cloned().collect()))
        .map(|(num, _)| num)
        .collect::<Vec<_>>();
    
    if gears.len() != 2 {
        return 0
    }
    gears[0]*gears[1]
}

fn part2(numbers: &Vec<(u32, Vec<(i32, i32)>)>, symbols: &HashMap<(i32, i32), char>) -> u32 {
    symbols.iter().filter(|(_, &c)| c == '*')
        .map(|(&pos, _)| gear_ratio(pos, numbers))
        .sum()
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Input file not found");
    let mut numbers: Vec<(u32, Vec<(i32, i32)>)> = Vec::new();
    let mut symbols: HashMap<(i32, i32), char> = HashMap::new();

    for (x, line) in data.lines().enumerate() {
        let mut positions: Vec<(i32, i32)> = Vec::new();
        let mut number = 0;

        for (y, c) in line.chars().enumerate() {
            let pos = (x as i32, y as i32);
            if c.is_digit(10) {
                number *= 10;
                number += c.to_digit(10).unwrap();
                positions.push(pos);
            } else {
                if number != 0 {
                    numbers.push((number, positions));
                    number = 0;
                    positions = Vec::new();
                }
                if c != '.' {
                    symbols.insert(pos, c);
                }
            }
        }

        if number != 0 {
            numbers.push((number, positions));
        }
    }
    
    println!("Part 1: {}", part1(&numbers, &symbols));
    println!("Part 2: {}", part2(&numbers, &symbols));
}