use std::{collections::HashSet, fs};

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag,
    character::complete::{multispace1, space1, u64},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::delimited,
};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day10_part1(&contents);
    println!("Day10 part 1 result: {result}");
}

fn day10_part1(input: &str) -> usize {
    let (_, machines) = read_input(input).unwrap();
    machines.iter().map(Machine::get_minimum_presses).sum()
}

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl Machine {
    fn get_minimum_presses(&self) -> usize {
        Machine::get_minimum_presses_recursive(self.lights.clone(), &self.buttons, 0)
    }

    fn get_minimum_presses_recursive(
        lights: Vec<bool>,
        buttons: &[Vec<usize>],
        path_length: usize,
    ) -> usize {
        // Today was a good day to be a pirate
        if path_length > 6 {
            return usize::MAX;
        }
        let light_to_change = match lights.iter().position(|light| *light) {
            Some(index) => index,
            None => return 0,
        };
        buttons
            .iter()
            .filter(|button| button.contains(&light_to_change))
            .map(|button| {
                let mut new_lights = lights.clone();
                for &i in button {
                    new_lights[i] = !new_lights[i];
                }
                Machine::get_minimum_presses_recursive(new_lights, buttons, path_length + 1)
                    .saturating_add(1)
            })
            .min()
            .unwrap_or(usize::MAX)
    }
}

fn read_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(
        multispace1,
        map(
            (read_lights, space1, read_buttons, space1, read_joltages),
            |(lights, _, buttons, _, joltages)| Machine {
                lights,
                buttons,
                joltages,
            },
        ),
    )
    .parse_complete(input)
}

fn read_lights(input: &str) -> IResult<&str, Vec<bool>> {
    delimited(
        tag("["),
        many1(alt((map(tag("."), |_| false), map(tag("#"), |_| true)))),
        tag("]"),
    )
    .parse_complete(input)
}

fn read_buttons(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(
        space1,
        delimited(
            tag("("),
            separated_list1(tag(","), map(u64, |n| n as usize)),
            tag(")"),
        ),
    )
    .parse_complete(input)
}

fn read_joltages(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        tag("{"),
        separated_list1(tag(","), map(u64, |n| n as usize)),
        tag("}"),
    )
    .parse_complete(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day10_part1(&contents);
        assert_eq!(result, 7);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day10_part1(&contents);
        assert_eq!(result, 399);
    }
}
