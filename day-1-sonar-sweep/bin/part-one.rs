const INPUT: &str = include_str!("../../inputs/day-1-sonar-sweep.txt");

fn main() {
    let mut previous = 0;
    let mut count_increased = 0;

    for current in INPUT.lines().map(|v| v.parse::<i32>().unwrap()) {
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
