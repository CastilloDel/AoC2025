use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day03_part1(&contents);
    println!("Day03 part 1 result: {result}");
    let result = day03_part2(&contents);
    println!("Day03 part 2 result: {result}");
}

fn day03_part1(input: &str) -> usize {
    read_battery_banks(input)
        .iter()
        .map(|battery| battery.get_largest_joltage(2))
        .sum()
}

fn day03_part2(input: &str) -> usize {
    read_battery_banks(input)
        .iter()
        .map(|battery| battery.get_largest_joltage(12))
        .sum()
}

struct BatteryBank {
    batteries: Vec<u8>,
}

impl BatteryBank {
    fn get_largest_joltage(&self, batteries_on: usize) -> usize {
        let mut joltage = 0;
        let mut available_batteries = self.batteries.as_slice();
        for remaning_choices in (0..batteries_on).rev() {
            let new_digit = *available_batteries[0..available_batteries.len() - remaning_choices]
                .iter()
                .max()
                .unwrap();
            // Ensure we get the first instance of the max number
            let new_digit_index = available_batteries
                .iter()
                .position(|&n| n == new_digit)
                .unwrap();
            joltage = joltage * 10 + new_digit as usize;
            available_batteries = &available_batteries[new_digit_index + 1..];
        }
        joltage
    }
}

fn read_battery_banks(input: &str) -> Vec<BatteryBank> {
    input
        .lines()
        .map(|line| BatteryBank {
            batteries: line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day03_part1(&contents);
        assert_eq!(result, 357);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day03_part1(&contents);
        assert_eq!(result, 17193);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day03_part2(&contents);
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day03_part2(&contents);
        assert_eq!(result, 171297349921310);
    }
}
