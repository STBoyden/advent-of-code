use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let input_file = File::open("./inputs/day-1-sonar-sweep.txt")
        .expect("Could not find ./inputs/day-1-sonar-sweep.txt");
    let reader = BufReader::new(input_file);
    let mut count_increased = 0;
    let mut previous_sum = 0;
    let lines = reader.lines().map(|x| x.unwrap()).collect::<Vec<_>>();

    // let mut line_iter = reader.lines().peekable();

    for (index, line) in lines.iter().enumerate() {
        let n0 = line.parse::<i32>().expect("Could not parse line as i32");
        let n1 = lines
            .get(index + 1)
            .unwrap_or(&"0".into())
            .parse::<i32>()
            .unwrap();
        let n2 = lines
            .get(index + 2)
            .unwrap_or(&"0".into())
            .parse::<i32>()
            .unwrap();

        let current_sum = n0 + n1 + n2;

        if previous_sum == 0 {
            println!("{} (N/A - no previous measurement", current_sum);
        } else if previous_sum < current_sum {
            count_increased += 1;
            println!("{} (increased)", current_sum);
        } else {
            println!("{} (decreased)", current_sum);
        }

        previous_sum = current_sum;
    }

    println!("Total increased: {}", count_increased);
}
