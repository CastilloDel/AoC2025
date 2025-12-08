use std::{collections::HashSet, fs};

use nom::{
    IResult, Parser, bytes::tag, character::complete::multispace1, character::complete::u64,
    combinator::map, multi::separated_list1,
};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day08_part1(&contents, 1000);
    println!("Day08 part 1 result: {result}");
    let result = day08_part2(&contents);
    println!("Day08 part 2 result: {result}");
}

fn day08_part1(input: &str, number_of_links: usize) -> usize {
    let (_, boxes) = read_input(input).unwrap();
    let mut box_pairs = get_pairs(&boxes);
    box_pairs.sort_by(|(a, b), (c, d)| a.distance_to(b).total_cmp(&c.distance_to(d)));
    let box_pairs = box_pairs
        .into_iter()
        .take(number_of_links)
        .collect::<Vec<_>>();
    let mut groups: Vec<HashSet<BoxPosition>> = Vec::new();
    for pair in box_pairs {
        add_link(&mut groups, pair);
    }
    let mut group_sizes = groups.iter().map(|group| group.len()).collect::<Vec<_>>();
    group_sizes.sort_by(|a, b| b.cmp(a));
    group_sizes.iter().take(3).product()
}

fn day08_part2(input: &str) -> u64 {
    let (_, boxes) = read_input(input).unwrap();
    let mut box_pairs = get_pairs(&boxes);
    box_pairs.sort_by(|(a, b), (c, d)| a.distance_to(b).total_cmp(&c.distance_to(d)));
    let last_connection = get_last_connection(&box_pairs, boxes.len());
    last_connection.0.x * last_connection.1.x
}

fn get_last_connection(
    links: &[(BoxPosition, BoxPosition)],
    number_of_boxes: usize,
) -> (BoxPosition, BoxPosition) {
    let mut groups: Vec<HashSet<BoxPosition>> = Vec::new();
    for pair in links {
        add_link(&mut groups, *pair);
        if groups[0].len() == number_of_boxes {
            return *pair;
        }
    }
    unreachable!()
}

fn add_link(groups: &mut Vec<HashSet<BoxPosition>>, pair: (BoxPosition, BoxPosition)) {
    let related_groups = groups
        .iter()
        .enumerate()
        .filter(|(_, group)| group.contains(&pair.0) || group.contains(&pair.1))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    if related_groups.len() == 1 {
        let group = &mut groups[related_groups[0]];
        group.insert(pair.0);
        group.insert(pair.1);
    } else if related_groups.len() == 2 {
        let group_to_remove = groups.remove(related_groups[1]);
        let group = &mut groups[related_groups[0]];
        group.extend(&group_to_remove);
    } else {
        let mut new_group = HashSet::new();
        new_group.insert(pair.0);
        new_group.insert(pair.1);
        groups.push(new_group);
    }
}

fn get_pairs(boxes: &[BoxPosition]) -> Vec<(BoxPosition, BoxPosition)> {
    boxes
        .iter()
        .enumerate()
        .flat_map(|(index, box1)| {
            boxes
                .iter()
                .skip(index + 1)
                .map(|box2| (*box1, *box2))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BoxPosition {
    x: u64,
    y: u64,
    z: u64,
}

impl BoxPosition {
    fn distance_to(&self, other: &BoxPosition) -> f64 {
        ((self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)) as f64)
            .sqrt()
    }
}

fn read_input(input: &str) -> IResult<&str, Vec<BoxPosition>> {
    separated_list1(
        multispace1,
        map(separated_list1(tag(","), u64), |positions| BoxPosition {
            x: positions[0],
            y: positions[1],
            z: positions[2],
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
        let result = day08_part1(&contents, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day08_part1(&contents, 1000);
        assert_eq!(result, 175440);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day08_part2(&contents);
        assert_eq!(result, 25272);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day08_part2(&contents);
        assert_eq!(result, 3200955921);
    }
}
