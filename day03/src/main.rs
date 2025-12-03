use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day03_part1(&contents);
    println!("Day03 part 1 result: {result}");
}

fn day03_part1(input: &str) -> usize {
    read_battery_banks(input)
        .iter()
        .map(BatteryBank::get_largest_joltage)
        .sum()
}

struct BatteryBank {
    batteries: Vec<u8>,
}
impl BatteryBank {
    fn get_largest_joltage(&self) -> usize {
        let first_digit = self.batteries[0..self.batteries.len() - 1]
            .iter()
            .max()
            .unwrap();
        // Ensure we get the first instance of the max number
        let first_digit_index = self
            .batteries
            .iter()
            .position(|n| n == first_digit)
            .unwrap();
        let second_digit = self.batteries[first_digit_index + 1..]
            .iter()
            .max()
            .unwrap();
        (first_digit * 10 + second_digit) as usize
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
}
