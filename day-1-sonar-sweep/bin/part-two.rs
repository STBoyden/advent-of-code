const INPUT: &str = include_str!("../../inputs/day-1-sonar-sweep.txt");

fn main() {
    let mut count_increased = 0;
    let mut previous_sum = 0;

    let iter = INPUT
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for (index, current) in INPUT.lines().map(|x| x.parse::<i32>().unwrap()).enumerate() {
        let n1 = iter.get(index + 1).unwrap_or(&0);
        let n2 = iter.get(index + 2).unwrap_or(&0);

        let current_sum = current + n1 + n2;

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
