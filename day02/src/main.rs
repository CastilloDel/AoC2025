use std::{fs, ops::Range};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day02_part1(&contents);
    println!("Day02 part 1 result: {result}");
}

fn day02_part1(input: &str) -> usize {
    read_ranges(input)
        .into_iter()
        .flat_map(get_invalid_ids)
        .sum()
}

fn get_invalid_ids(range: Range<usize>) -> Vec<usize> {
    range.filter(|n: &usize| is_invalid(*n)).collect()
}

fn is_invalid(n: usize) -> bool {
    let magnitude = 10_usize.pow(n.ilog10().div_ceil(2));
    n / magnitude == n % magnitude
}

fn read_ranges(input: &str) -> Vec<Range<usize>> {
    input
        .trim()
        .split(",")
        .map(|range| {
            let bounds: Vec<usize> = range.split("-").map(|n| n.parse().unwrap()).collect();
            bounds[0]..bounds[1] + 1
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day02_part1(&contents);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day02_part1(&contents);
        assert_eq!(result, 23560874270);
    }
}
