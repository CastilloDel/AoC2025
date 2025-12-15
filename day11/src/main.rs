use nom::{
    IResult, Parser,
    bytes::tag,
    character::complete::{alpha1, multispace1, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};
use std::{collections::HashMap, fs};

const START: &str = "you";
const END: &str = "out";

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day11_part1(&contents);
    println!("Day11 part 1 result: {result}");
}

fn day11_part1(input: &str) -> usize {
    let (_, connections) = read_input(input).unwrap();
    count_paths(&connections, START)
}

fn count_paths(connections: &HashMap<&str, Vec<&str>>, start: &str) -> usize {
    if start == END {
        return 1;
    }
    connections[start]
        .iter()
        .map(|next| count_paths(connections, next))
        .sum()
}

type DeviceConnections<'a> = HashMap<&'a str, Vec<&'a str>>;

fn read_input(input: &str) -> IResult<&str, DeviceConnections<'_>> {
    map(
        separated_list1(
            multispace1,
            separated_pair(alpha1, tag(": "), separated_list1(space1, alpha1)),
        ),
        |devices| devices.into_iter().collect(),
    )
    .parse_complete(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day11_part1(&contents);
        assert_eq!(result, 5);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day11_part1(&contents);
        assert_eq!(result, 772);
    }
}
