use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let input_file = File::open("./inputs/day-1-sonar-sweep.txt")
        .expect("Could not find ./inputs/day-1-sonar-sweep.txt");
    let reader = BufReader::new(input_file);
    let mut count_increased = 0;
    let mut previous = 0;

    for line in reader.lines() {
        if line.is_err() {
            break;
        }

        let line = line.unwrap();
        let current = line
            .parse::<i32>()
            .expect("Could not parse string into i32");

        if previous == 0 {
            println!("{} (N/A - no previous measurement)", current);
        } else if previous < current {
            count_increased += 1;
            println!("{} (increased)", current);
        } else {
            println!("{} (decreased)", current);
        }

        previous = current;
    }

    println!("Total increased: {}", count_increased);
}
