#![allow(unstable_name_collisions)]

use common_macros::hash_map;
use itertools::Itertools;

fn part_one(input: &str) -> i32 {
    input
        .lines()
        .to_owned()
        .map(|line| line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>())
        .filter_map(|digits| (digits.first().cloned()).zip(digits.last().cloned()))
        .filter_map(|(first, last)| format!("{first}{last}").parse::<i32>().ok())
        .sum()
}

fn part_two(input: &str) -> i32 {
    let number_map = hash_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
    };

    let lines = input
        .lines()
        .map(|line| {
            let mut line = String::from(line);
            let mut ranges = vec![];

            number_map.keys().for_each(|key| {
                ranges.append(
                    &mut line
                        .match_indices(key)
                        .filter_map(|(index, mtch)| {
                            Some(index..index + mtch.chars().size_hint().0)
                                .zip(number_map.get(mtch))
                        })
                        .collect::<Vec<_>>(),
                );
            });

            ranges.sort_by(|(range_a, _), (range_b, _)| range_a.start.cmp(&range_b.start));

            if ranges.len() > 1 {
                let first = ranges.first();
                let last = ranges.last();

                if let Some(((first_range, first_number), (last_range, last_number))) =
                    first.zip(last)
                {
                    line.replace_range(first_range.clone(), &format!("{first_number}"));
                    line.replace_range(last_range.clone(), &format!("{last_number}"));
                }
            } else if let Some((range, number)) = ranges.first() {
                line.replace_range(range.clone(), &format!("{number}"));
            }

            line
        })
        .intersperse(String::from("\n"))
        .collect::<Vec<_>>()
        .join("");

    part_one(&lines)
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input))
}
