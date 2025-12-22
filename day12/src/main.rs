use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag,
    character::complete::{alphanumeric1, multispace1, space1, u64},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{pair, preceded, separated_pair},
};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day12_part1(&contents);
    println!("Day12 part 1 result: {result}");
}

fn day12_part1(input: &str) -> usize {
    let (_, problem) = read_input(input).unwrap();
    problem
        .regions
        .iter()
        .filter(|region| region.is_solvable(&problem.shapes))
        .count()
}

type Shape = Vec<Vec<bool>>;

#[derive(Debug, Clone)]
struct Region {
    m: u64,
    n: u64,
    present_allocations: Vec<u64>,
}

impl Region {
    fn is_solvable(&self, shapes: &[Vec<Vec<bool>>]) -> bool {
        let size: u64 = shapes
            .iter()
            .enumerate()
            .map(|(i, shape)| {
                self.present_allocations[i]
                    * shape
                        .iter()
                        .flat_map(|row| row.iter())
                        .filter(|c| **c)
                        .count() as u64
            })
            .sum();
        return size <= self.n * self.m;
    }
}

struct Problem {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

fn read_input(input: &str) -> IResult<&str, Problem> {
    map(
        separated_pair(
            separated_list1(multispace1, read_shape),
            multispace1,
            separated_list1(multispace1, read_region),
        ),
        |(shapes, regions)| Problem { shapes, regions },
    )
    .parse_complete(input)
}

fn read_shape(input: &str) -> IResult<&str, Shape> {
    preceded(
        pair(alphanumeric1, tag(":\n")),
        separated_list1(
            multispace1,
            many1(map(alt((tag("#"), tag("."))), |c| c == "#")),
        ),
    )
    .parse_complete(input)
}

fn read_region(input: &str) -> IResult<&str, Region> {
    map(
        separated_pair(
            separated_pair(u64, tag("x"), u64),
            tag(": "),
            separated_list1(space1, u64),
        ),
        |((n, m), present_allocations)| Region {
            m,
            n,
            present_allocations,
        },
    )
    .parse_complete(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day12_part1(&contents);
        assert_eq!(result, 422);
    }
}
