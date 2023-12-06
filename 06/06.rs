use std::fs;


fn get_ways_cnt(time: i64, distance: i64) -> u64 {
    // x^2 - time*x + distance < 0 
    let discr: f64 = (time*time - 4*distance) as f64;
    if discr < 0.0 {
        return 0;
    }

    let up = (time as f64 + discr.sqrt())/2.0 - 1.0;
    let bot = (time as f64 - discr.sqrt())/2.0 + 1.0;
    (up.ceil() - bot.floor() + 1.0) as u64
}

fn main() {
    let text = fs::read_to_string("input.txt").expect("Input file not found");
    let data: Vec<Vec<i64>> = text.lines()
        .map(|line| line
            .split_whitespace()
            .skip(1)
            .map(|item| item.parse().unwrap())
            .collect())
        .collect();

    let games: Vec<(i64, i64)> = data[0].iter()
        .zip(data[1].iter())
        .map(|(&a, &b)| (a, b))
        .collect();
    
    let one_game: Vec<i64> = text.lines()
        .map(|line| line.split(":").last().unwrap()
            .replace(" ", "")
            .parse().unwrap()
        )
        .collect();

    let part1 = games.iter().fold(1, |acc, game| acc * get_ways_cnt(game.0, game.1));
    let part2 = get_ways_cnt(one_game[0], one_game[1]);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}