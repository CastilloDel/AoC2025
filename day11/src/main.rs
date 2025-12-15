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
const START_PART2: &str = "svr";
const FFT: &str = "fft";
const DAC: &str = "dac";

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day11_part1(&contents);
    println!("Day11 part 1 result: {result}");
    let result = day11_part2(&contents);
    println!("Day11 part 2 result: {result}");
}

fn day11_part1(input: &str) -> usize {
    let (_, connections) = read_input(input).unwrap();
    count_paths(&connections, START)
}

fn day11_part2(input: &str) -> usize {
    let (_, connections) = read_input(input).unwrap();
    count_paths_part2(
        &connections,
        START_PART2,
        PathState::Invalid,
        &mut HashMap::new(),
    )
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum PathState {
    Invalid,
    Ffted,
    Daced,
    Complete,
}

fn count_paths_part2<'a>(
    connections: &HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    mut state: PathState,
    cache: &mut HashMap<(&'a str, PathState), usize>,
) -> usize {
    if start == END {
        return if state == PathState::Complete { 1 } else { 0 };
    }
    if cache.contains_key(&(start, state)) {
        return cache[&(start, state)];
    }
    state = match (state, start) {
        (PathState::Invalid, FFT) => PathState::Ffted,
        (PathState::Invalid, DAC) => PathState::Daced,
        (PathState::Ffted, DAC) => PathState::Complete,
        (PathState::Daced, FFT) => PathState::Complete,
        _ => state,
    };
    let result = connections[start]
        .iter()
        .map(|next| count_paths_part2(connections, next, state, cache))
        .sum();
    cache.insert((start, state), result);
    result
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

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input2").unwrap();
        let result = day11_part2(&contents);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day11_part2(&contents);
        assert_eq!(result, 423227545768872);
    }
}
