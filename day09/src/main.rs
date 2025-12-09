use std::fs;

use nom::{
    IResult, Parser,
    bytes::tag,
    character::complete::{multispace1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day09_part1(&contents);
    println!("Day09 part 1 result: {result}");
}

fn day09_part1(input: &str) -> u64 {
    let (_, tiles) = read_input(input).unwrap();
    let mut tile_pairs = get_pairs(&tiles);
    tile_pairs.sort_by_cached_key(|(a, b)| a.get_area_of_rectangle(b));
    let biggest_rectangle = tile_pairs.last().unwrap();
    biggest_rectangle
        .0
        .get_area_of_rectangle(&biggest_rectangle.1)
}

fn get_pairs(tiles: &[Position]) -> Vec<(Position, Position)> {
    tiles
        .iter()
        .enumerate()
        .flat_map(|(index, box1)| {
            tiles
                .iter()
                .skip(index + 1)
                .map(|box2| (*box1, *box2))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: u64,
    y: u64,
}

impl Position {
    fn get_area_of_rectangle(&self, other: &Position) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn read_input(input: &str) -> IResult<&str, Vec<Position>> {
    separated_list1(
        multispace1,
        map(separated_pair(u64, tag(","), u64), |(x, y)| Position {
            x,
            y,
        }),
    )
    .parse_complete(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day09_part1(&contents);
        assert_eq!(result, 50);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day09_part1(&contents);
        assert_eq!(result, 4738108384);
    }
}
