const INPUT_FILE: &str = include_str!("../../inputs/day-2-dive.txt");

fn main() {
    let mut depth = 0;
    let mut horizontal = 0;

    INPUT_FILE
        .lines()
        .map(|line| {
            let vec = line.split_whitespace().collect::<Vec<_>>();
            (vec[0], vec[1].parse::<i32>().unwrap())
        })
        .for_each(|(instruction, value)| match instruction {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => (),
        });

    println!("{}", depth * horizontal);
}
