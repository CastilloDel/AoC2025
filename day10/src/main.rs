use itertools::Itertools;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag,
    character::complete::{multispace1, space1, u64},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::delimited,
};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day10_part1(&contents);
    println!("Day10 part 1 result: {result}");
    let result = day10_part2(&contents);
    println!("Day10 part 2 result: {result}");
}

fn day10_part1(input: &str) -> usize {
    let (_, machines) = read_input(input).unwrap();
    machines.iter().map(Machine::get_minimum_presses).sum()
}

fn day10_part2(input: &str) -> usize {
    let (_, machines) = read_input(input).unwrap();
    machines
        .iter()
        .map(|machine| machine.get_minimum_presses_joltage())
        .sum()
}

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl Machine {
    fn get_minimum_presses(&self) -> usize {
        Machine::get_possible_minimum_presses(self.lights.clone(), &self.buttons)
            .iter()
            .map(|possible_presses| possible_presses.len())
            .min()
            .unwrap()
    }

    fn get_possible_minimum_presses(lights: Vec<bool>, buttons: &[Vec<usize>]) -> Vec<Vec<usize>> {
        let mut state = 0;
        for (index, light) in lights.iter().enumerate() {
            if *light {
                state |= 1 << index;
            }
        }
        let buttons_operations = buttons
            .iter()
            .map(|joltage_indexes| {
                joltage_indexes
                    .iter()
                    .fold(0, |acc, joltage_index| acc | 1 << joltage_index)
            })
            .collect::<Vec<_>>();
        (0..=buttons.len())
            .flat_map(|length| (0..buttons.len()).combinations(length))
            .filter(|combination| {
                combination
                    .into_iter()
                    .fold(0, |acc, index| acc ^ buttons_operations[*index])
                    == state
            })
            .collect()
    }

    fn get_minimum_presses_joltage(&self) -> usize {
        Machine::get_minimum_presses_joltage_recursive(self.joltages.clone(), &self.buttons)
            .unwrap()
    }

    fn get_minimum_presses_joltage_recursive(
        joltages: Vec<usize>,
        buttons: &[Vec<usize>],
    ) -> Option<usize> {
        if joltages.iter().all(|j| *j == 0) {
            return Some(0);
        }
        let lights = joltages
            .iter()
            .map(|joltage| joltage % 2 != 0)
            .collect::<Vec<_>>();
        let possible_presses = Machine::get_possible_minimum_presses(lights, buttons);
        possible_presses
            .iter()
            .filter_map(|presses| {
                let mut new_joltages = joltages.clone();
                for press in presses {
                    for j in &buttons[*press] {
                        if new_joltages[*j] == 0 {
                            return None;
                        };
                        new_joltages[*j] -= 1;
                    }
                }
                for joltage in new_joltages.iter_mut() {
                    *joltage /= 2;
                }
                Machine::get_minimum_presses_joltage_recursive(new_joltages, buttons)
                    .map(|next_presses| 2 * next_presses + presses.len())
            })
            .min()
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

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day10_part2(&contents);
        assert_eq!(result, 33);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day10_part2(&contents);
        assert_eq!(result, 15631);
    }
}
