use std::fs;

fn  find_number(line: &str) -> u32 {
    let nums: Vec<u32> = line.chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let first = nums.first().unwrap();
    let last = nums.last().unwrap();

    first*10 + last
}

fn convert(line: &str) -> String {
    let replacers = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    // replace first item
    let first_pos = line.chars().position(|c| c.is_digit(10)).unwrap_or(line.len());
    let to_replace = replacers.iter()
        .map(|item| (line.find(item.0).unwrap_or(line.len()), item))
        .filter(|(pos, _)| *pos < first_pos)
        .min_by_key(|&(pos, _)| pos);

    let mut data = line.to_string();
    if to_replace != None {
        let replacer = to_replace.unwrap().1;
        data = line.to_string().replacen(replacer.0, replacer.1, 1);
    }

    // replace last item
    let last_pos = data.len() - data.chars().rev().position(|c| c.is_digit(10)).unwrap_or(data.len()) - 1;
    let to_replace = replacers.iter()
        .map(|item| (data.rfind(item.0).unwrap_or(0), item))
        .filter(|(pos, _)| *pos > last_pos)
        .max_by_key(|&(pos, _)| pos);

    if to_replace != None {
        let replacer = to_replace.unwrap().1;
        let pos = to_replace.unwrap().0;
        let first: String = data.chars().take(pos).collect();
        let last: String = data.chars().skip(pos).collect();
        data = first.to_string() + &last.replacen(replacer.0, replacer.1, 1);
    }

    data
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Input file not found");
    let lines = data.lines();

    let part1: u32 = lines.clone().map(|line| find_number(line)).sum();
    let part2: u32 = lines.clone().map(|line| find_number(&convert(line))).sum();
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}