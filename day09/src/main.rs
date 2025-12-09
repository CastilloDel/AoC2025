use std::{
    cmp::{max, min},
    collections::BTreeMap,
    fs,
    ops::RangeInclusive,
};

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
    let result = day09_part2(&contents);
    println!("Day09 part 2 result: {result}");
}

fn day09_part1(input: &str) -> usize {
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

fn day09_part2(input: &str) -> usize {
    let read_input = read_input(input).unwrap();
    let (_, tiles) = read_input;
    let mut links = tiles.iter().zip(tiles.iter().skip(1)).collect::<Vec<_>>();
    links.push((tiles.last().unwrap(), &tiles[0]));
    // We keep only the horizontal links
    links.retain(|(tile1, tile2)| tile1.y == tile2.y);
    links.sort_by_cached_key(|link| link.0.y);
    let acceptable_ranges = get_acceptable_ranges(links);

    let mut tile_pairs = get_pairs(&tiles);
    tile_pairs.sort_by_cached_key(|(a, b)| b.get_area_of_rectangle(a));
    let biggest_rectangle = tile_pairs
        .iter()
        .rev() // First the ones with the largest area
        .find(|(a, b)| check_acceptable(&acceptable_ranges, a, b))
        .unwrap();
    biggest_rectangle
        .0
        .get_area_of_rectangle(&biggest_rectangle.1)
}

fn get_acceptable_ranges(
    links: Vec<(&Position, &Position)>,
) -> BTreeMap<usize, Vec<RangeInclusive<usize>>> {
    let mut acceptable_ranges: BTreeMap<usize, Vec<RangeInclusive<usize>>> = BTreeMap::new();
    acceptable_ranges.insert(0, vec![]);
    for link in links {
        let start = min(link.0.x, link.1.x);
        let end = max(link.0.x, link.1.x);
        let previous_ranges = acceptable_ranges.last_key_value().unwrap().1.clone();
        let ranges = acceptable_ranges
            .remove(&link.0.y)
            .unwrap_or(previous_ranges);
        let mut new_ranges: Vec<RangeInclusive<usize>> = vec![];
        let before_link = ranges
            .iter()
            .filter(|range| *range.end() < start)
            .cloned()
            .collect::<Vec<RangeInclusive<usize>>>();
        new_ranges.extend(before_link);
        let colliding_with_link = ranges
            .iter()
            .filter(|range| range.contains(&start) || range.contains(&end))
            .cloned()
            .collect::<Vec<RangeInclusive<usize>>>();
        if colliding_with_link.is_empty() {
            new_ranges.push(start..=end);
        } else if colliding_with_link.len() == 1 {
            let range = &colliding_with_link[0];
            if range.contains(&start) && range.contains(&end) {
                if start != *range.start() {
                    new_ranges.push((*range.start())..=min(start, *range.end()));
                } else if end != *range.end() {
                    new_ranges.push(max(end, *range.start())..=(*range.end()));
                }
                // If both start and range match, the range just dissappears
            } else if range.contains(&start) || range.contains(&end) {
                new_ranges.push(min(start, *range.start())..=max(end, *range.end()));
            }
        } else {
            let range1 = &colliding_with_link[0];
            let range2 = &colliding_with_link[1];
            new_ranges
                .push(min(*range1.start(), *range2.start())..=max(*range1.end(), *range2.end()));
        }
        new_ranges.extend(ranges.iter().filter(|range| *range.start() > end).cloned());
        acceptable_ranges.insert(link.0.y, new_ranges);
    }
    acceptable_ranges
}

fn check_acceptable(
    acceptables_ranges: &BTreeMap<usize, Vec<RangeInclusive<usize>>>,
    a: &Position,
    b: &Position,
) -> bool {
    let top = min(a.y, b.y);
    let bottom = max(a.y, b.y);
    let start = min(a.x, b.x);
    let end = max(a.x, b.x);
    for (_, ranges) in acceptables_ranges.range(top..bottom) {
        if !ranges
            .iter()
            .any(|range| range.contains(&start) && range.contains(&end))
        {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn get_area_of_rectangle(&self, other: &Position) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn read_input(input: &str) -> IResult<&str, Vec<Position>> {
    separated_list1(
        multispace1,
        map(separated_pair(u64, tag(","), u64), |(x, y)| Position {
            x: x as usize,
            y: y as usize,
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

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day09_part2(&contents);
        assert_eq!(result, 24);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day09_part2(&contents);
        assert_eq!(result, 1513792010);
    }
}
