use std::{
    collections::HashMap,
    fs,
    ops::{Index, IndexMut},
};

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag,
    character::complete::multispace1,
    combinator::map,
    multi::{many1, separated_list1},
};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day07_part1(&contents);
    println!("Day07 part 1 result: {result}");
    let result = day07_part2(&contents);
    println!("Day07 part 2 result: {result}");
}

fn day07_part1(input: &str) -> usize {
    let (_, mut matrix) = read_input(input).unwrap();
    let start_pos = matrix
        .iter()
        .find(|&pos| matrix[pos] == Cell::Start)
        .unwrap();
    let next = matrix
        .get_next_position(start_pos, Direction::Down)
        .unwrap();
    throw_beam(&mut matrix, next)
}

fn throw_beam(matrix: &mut Matrix<Cell>, pos: Position) -> usize {
    match matrix[pos] {
        Cell::Empty => {
            matrix[pos] = Cell::Beam;
            matrix
                .get_next_position(pos, Direction::Down)
                .map(|next| throw_beam(matrix, next))
                .unwrap_or(0)
        }
        Cell::Splitter => {
            let right_count = matrix
                .get_next_position(pos, Direction::Right)
                .map(|next| throw_beam(matrix, next))
                .unwrap_or(0);
            let left_count = matrix
                .get_next_position(pos, Direction::Left)
                .map(|next| throw_beam(matrix, next))
                .unwrap_or(0);
            right_count + left_count + 1
        }
        Cell::Beam => 0,
        Cell::Start => unreachable!(),
    }
}

fn day07_part2(input: &str) -> usize {
    let (_, mut matrix) = read_input(input).unwrap();
    let start_pos = matrix
        .iter()
        .find(|&pos| matrix[pos] == Cell::Start)
        .unwrap();
    let next = matrix
        .get_next_position(start_pos, Direction::Down)
        .unwrap();
    throw_quantum_beam(&mut matrix, next, &mut HashMap::new()) + 1
}

fn throw_quantum_beam(
    matrix: &mut Matrix<Cell>,
    pos: Position,
    cache: &mut HashMap<Position, usize>,
) -> usize {
    if cache.contains_key(&pos) {
        return cache[&pos];
    }
    let value = match matrix[pos] {
        Cell::Empty | Cell::Beam => {
            matrix[pos] = Cell::Beam;
            matrix
                .get_next_position(pos, Direction::Down)
                .map(|next| throw_quantum_beam(matrix, next, cache))
                .unwrap_or(0)
        }
        Cell::Splitter => {
            let right_count = matrix
                .get_next_position(pos, Direction::Right)
                .map(|next| throw_quantum_beam(matrix, next, cache))
                .unwrap_or(0);
            let left_count = matrix
                .get_next_position(pos, Direction::Left)
                .map(|next| throw_quantum_beam(matrix, next, cache))
                .unwrap_or(0);
            right_count + left_count + 1
        }
        Cell::Start => unreachable!(),
    };
    cache.insert(pos, value);
    value
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Start,
    Empty,
    Splitter,
    Beam,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    #[allow(unused)]
    Up,
    Right,
    Down,
    Left,
}

type Position = (usize, usize);

#[derive(Debug)]
struct Matrix<T> {
    inner: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    fn get_next_position(&self, pos: Position, direction: Direction) -> Option<Position> {
        let change = match direction {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };
        if (pos.0 == 0 && change.0 == -1)
            || (pos.1 == 0 && change.1 == -1)
            || (pos.0 == self.m() - 1 && change.0 == 1)
            || (pos.1 == self.n() - 1 && change.1 == 1)
        {
            return None;
        }
        Some((
            (pos.0 as isize + change.0) as usize,
            (pos.1 as isize + change.1) as usize,
        ))
    }

    fn m(&self) -> usize {
        self.inner.len()
    }

    fn n(&self) -> usize {
        self.inner[0].len()
    }

    pub fn iter(&self) -> impl Iterator<Item = Position> {
        (0..self.m()).flat_map(|i| (0..self.n()).map(move |j| (i, j)))
    }
}

impl<T> Index<Position> for Matrix<T> {
    type Output = T;

    fn index(&self, pos: Position) -> &Self::Output {
        &self.inner[pos.0][pos.1]
    }
}

impl<T> IndexMut<Position> for Matrix<T> {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self.inner[pos.0][pos.1]
    }
}

fn read_input(input: &str) -> IResult<&str, Matrix<Cell>> {
    map(
        separated_list1(
            multispace1,
            many1(alt((
                map(tag("S"), |_| Cell::Start),
                map(tag("^"), |_| Cell::Splitter),
                map(tag("."), |_| Cell::Empty),
            ))),
        ),
        |inner| Matrix { inner },
    )
    .parse_complete(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day07_part1(&contents);
        assert_eq!(result, 21);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day07_part1(&contents);
        assert_eq!(result, 1609);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day07_part2(&contents);
        assert_eq!(result, 40);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day07_part2(&contents);
        assert_eq!(result, 12472142047197);
    }
}
