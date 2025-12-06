use std::fs;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag,
    character::{
        complete::{digit1, line_ending, multispace1, space1},
        satisfy,
    },
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day06_part1(&contents);
    println!("Day06 part 1 result: {result}");
    let result = day06_part2(&contents);
    println!("Day06 part 2 result: {result}");
}

fn day06_part1(input: &str) -> u64 {
    let (_, (numbers, operations)) = read_input(input).unwrap();
    numbers
        .into_iter()
        .reduce(|results, new_operands| {
            results
                .iter()
                .zip(new_operands)
                .zip(operations.iter())
                .map(|((a, b), op)| match op {
                    Operation::Sum => a + b,
                    Operation::Multiplication => a * b,
                })
                .collect::<Vec<u64>>()
        })
        .unwrap()
        .into_iter()
        .sum()
}
fn day06_part2(input: &str) -> u64 {
    let (_, (numbers, operations)) = read_input_part2(input).unwrap();
    numbers
        .into_iter()
        .zip(operations.iter())
        .map(|(operands, operation)| {
            operands
                .into_iter()
                .reduce(|total, operand| match operation {
                    Operation::Sum => total + operand,
                    Operation::Multiplication => total * operand,
                })
                .unwrap()
        })
        .sum()
}

enum Operation {
    Sum,
    Multiplication,
}

fn read_input(input: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<Operation>)> {
    separated_pair(
        separated_list1(multispace1, separated_list1(space1, read_number)),
        multispace1,
        separated_list1(space1, read_operation),
    )
    .parse_complete(input)
}
fn read_input_part2(input: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<Operation>)> {
    separated_pair(
        map(
            separated_list1(
                line_ending,
                many1(satisfy(|c| c.is_ascii_digit() || c == ' ')),
            ),
            read_cephalopod_numbers,
        ),
        line_ending,
        separated_list1(space1, read_operation),
    )
    .parse_complete(input)
}

fn read_cephalopod_numbers(lines: Vec<Vec<char>>) -> Vec<Vec<u64>> {
    let mut numbers = Vec::new();
    let mut current_numbers = Vec::new();
    for column in 0..lines[0].len() {
        let mut is_column_spaces = true;
        let mut number = String::new();
        for line in lines.iter() {
            let new_char = line[column];
            number.push(new_char);
            is_column_spaces = is_column_spaces && new_char == ' ';
        }
        if is_column_spaces {
            numbers.push(current_numbers);
            current_numbers = Vec::new();
        } else {
            current_numbers.push(number.trim().parse().unwrap());
        }
    }
    numbers.push(current_numbers);
    numbers
}

fn read_number(input: &str) -> IResult<&str, u64> {
    map(digit1, |s: &str| s.parse().unwrap()).parse_complete(input)
}

fn read_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        map(tag("+"), |_| Operation::Sum),
        map(tag("*"), |_| Operation::Multiplication),
    ))
    .parse_complete(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day06_part1(&contents);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day06_part1(&contents);
        assert_eq!(result, 5346286649122);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day06_part2(&contents);
        assert_eq!(result, 3263827);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day06_part2(&contents);
        assert_eq!(result, 10389131401929);
    }
}
