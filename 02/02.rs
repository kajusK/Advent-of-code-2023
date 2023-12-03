use std::fs;

fn is_possible(game: Vec<(u32, u32, u32)>) -> bool {
    game.iter().map(|round| round.0 <= 12 && round.1 <= 13 && round.2 <= 14).all(|c| c)
}

fn min_cubes(game: Vec<(u32, u32, u32)>) -> u32 {
    game.iter().fold([0, 0, 0], |acc, (a, b, c)| [acc[0].max(*a), acc[1].max(*b), acc[2].max(*c)])
        .iter().fold(1, |acc, x| acc*x)
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Input file not found");
    let mut part1 = 0;
    let mut part2 = 0;

    for line in data.lines() {
        let spl: Vec<&str> = line.split(":").collect();
        let game_num: u32 = spl.first().unwrap().split(" ").last().unwrap().parse().unwrap();
        let rounds = spl.last().unwrap().split(";");

        let data: Vec<(u32, u32, u32)> = rounds.map(|round| {
            let items = round.split(",")
                .map(|c| c.trim().split(" ").collect::<Vec<&str>>());
           
            items.map(|item| {
                let count: u32 = item.first().unwrap().parse().unwrap();
                let color = item.last().unwrap();
                if color.eq(&"red") {
                    (count, 0, 0)
                } else if color.eq(&"green") {
                    (0, count, 0)
                } else {
                    (0, 0, count)
                }
            }).fold((0, 0, 0), |acc, (a, b, c)| (acc.0 + a, acc.1 + b, acc.2 + c))
        }).collect();

        if is_possible(data.clone()) {
            part1 += game_num;
        }
        part2 += min_cubes(data);
    }

    println!("Part1: {part1}");
    println!("Part2: {part2}");
}