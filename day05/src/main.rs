use std::{collections::HashSet, fs, ops::RangeInclusive};

use nom::{
    IResult, Parser,
    bytes::tag,
    character::complete::{digit1, multispace1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day05_part1(&contents);
    println!("Day05 part 1 result: {result}");
}

fn day05_part1(input: &str) -> usize {
    let (_, (fresh_ranges, ingredients)) = read_input(input).unwrap();
    let mut rotten_ingredients = ingredients.iter().collect::<HashSet<_>>();
    for range in fresh_ranges {
        rotten_ingredients.retain(|ingredient| !range.contains(ingredient));
    }
    ingredients.len() - rotten_ingredients.len()
}

fn read_input(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    separated_pair(
        separated_list1(
            multispace1,
            map(
                separated_pair(read_number, tag("-"), read_number),
                |(a, b)| a..=b,
            ),
        ),
        multispace1,
        separated_list1(multispace1, read_number),
    )
    .parse_complete(input)
}

fn read_number(input: &str) -> IResult<&str, u64> {
    map(digit1, |s: &str| s.parse().unwrap()).parse_complete(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day05_part1(&contents);
        assert_eq!(result, 3);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day05_part1(&contents);
        assert_eq!(result, 638);
    }
}
