use std::{
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
    let result = day02_part1(&contents);
    println!("Day02 part 1 result: {result}");
}

fn day02_part1(input: &str) -> usize {
    let (_, matrix) = read_input(input).unwrap();
    (0..matrix.m())
        .map(|i| {
            (0..matrix.n())
                .map(|j| (i, j))
                .filter(|&pos| matrix[pos] == Cell::Paper)
                .filter(|&pos| {
                    matrix
                        .get_neighbors(pos)
                        .into_iter()
                        .filter(|&neighbour| matrix[neighbour] == Cell::Paper)
                        .count()
                        < 4
                })
                .count()
        })
        .sum()
}

#[derive(Debug, PartialEq)]
enum Cell {
    Paper,
    Empty,
}

type Position = (usize, usize);

#[derive(Debug)]
struct Matrix<T> {
    inner: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    fn get_neighbors(&self, pos: Position) -> Vec<Position> {
        vec![
            (1, 1),
            (1, 0),
            (1, -1),
            (0, 1),
            (0, -1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ]
        .into_iter()
        .filter(|change| {
            (pos.0 != 0 || change.0 != -1)
                && (pos.1 != 0 || change.1 != -1)
                && (pos.0 != self.m() - 1 || change.0 != 1)
                && (pos.1 != self.n() - 1 || change.1 != 1)
        })
        .map(|change: (isize, isize)| {
            (
                (pos.0 as isize + change.0) as usize,
                (pos.1 as isize + change.1) as usize,
            )
        })
        .collect()
    }

    fn m(&self) -> usize {
        self.inner.len()
    }

    fn n(&self) -> usize {
        self.inner[0].len()
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
                map(tag("."), |_| Cell::Empty),
                map(tag("@"), |_| Cell::Paper),
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
        let result = day02_part1(&contents);
        assert_eq!(result, 13);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day02_part1(&contents);
        assert_eq!(result, 1518);
    }
}
