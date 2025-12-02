use std::{fs, ops::Range};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = part1::day02(&contents);
    println!("Day02 part 1 result: {result}");
    let result = part2::day02(&contents);
    println!("Day02 part 2 result: {result}");
}

mod part1 {
    use super::*;

    pub(super) fn day02(input: &str) -> usize {
        read_ranges(input)
            .into_iter()
            .flat_map(get_invalid_ids)
            .sum()
    }

    fn get_invalid_ids(range: Range<usize>) -> Vec<usize> {
        range.filter(|n: &usize| is_invalid(*n)).collect()
    }

    fn is_invalid(n: usize) -> bool {
        let digits = n.ilog10() + 1;
        if digits % 2 != 0 {
            return false;
        }
        n / 10_usize.pow(digits / 2) == n % 10_usize.pow(digits / 2)
    }
}

mod part2 {
    use super::*;

    pub(super) fn day02(input: &str) -> usize {
        read_ranges(input)
            .into_iter()
            .flat_map(get_invalid_ids)
            .sum()
    }

    fn get_invalid_ids(range: Range<usize>) -> Vec<usize> {
        range.filter(|n: &usize| is_invalid(*n)).collect()
    }

    fn is_invalid(n: usize) -> bool {
        let digits = n.ilog10() + 1;
        if digits < 2 {
            return false;
        }
        for part_size in 1..=digits / 2 {
            if digits % part_size != 0 {
                continue;
            }
            let mut parts = (0..digits / part_size).map(|part_number| {
                n % 10_usize.pow((part_number + 1) * part_size)
                    / 10_usize.pow(part_number * part_size)
            });

            // We check if all parts are equal
            let first = parts.next().unwrap();
            if parts.all(|part| part == first) {
                return true;
            }
        }
        false
    }
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
        let result = part1::day02(&contents);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = part1::day02(&contents);
        assert_eq!(result, 23560874270);
    }
    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = part2::day02(&contents);
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = part2::day02(&contents);
        assert_eq!(result, 44143124633);
    }
}
