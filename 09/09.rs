use std::fs;

fn transfer_seq(data: &Vec<i32>) -> Vec<i32> {
    data.windows(2)
        .map(|items| items[1] - items[0])
        .collect()
}

fn predict(data: &Vec<i32>) -> (i32, i32) {
    let mut lasts: Vec<i32> = Vec::new();
    let mut firsts: Vec<i32> = Vec::new();
    let mut cur = data.clone();

    while !cur.iter().all(|x| *x == 0) {
        firsts.push(*cur.first().unwrap());
        lasts.push(*cur.last().unwrap());
        cur = transfer_seq(&cur);
    }
    (firsts.iter().rev().fold(0, |acc, x| x-acc),
     lasts.iter().rev().fold(0, |acc, x| acc+x))
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file not found");

    let data: Vec<Vec<i32>> = input.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect()
        })
        .collect();

    let predicts: Vec<(i32, i32)> = data.iter()
        .map(|vec| predict(vec))
        .collect();

    println!("Part 1: {}", predicts.iter().map(|x| x.1).sum::<i32>());
    println!("Part 2: {}", predicts.iter().map(|x| x.0).sum::<i32>());
}