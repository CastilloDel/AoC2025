use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day01_part1(&contents);
    println!("Day01 part 1 result: {result}");
    let result = day01_part2(&contents);
    println!("Day01 part 2 result: {result}");
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

struct Rotation {
    direction: Direction,
    number: usize,
}

fn day01_part1(input: &str) -> i64 {
    let rotations = read_rotations(input);
    let mut value = 50;
    let mut times_endend_in_zero = 0;
    for rotation in rotations {
        if rotation.direction == Direction::Left {
            value = (value + 100 - (rotation.number % 100)) % 100;
        } else {
            value = (value + rotation.number) % 100;
        }
        if value == 0 {
            times_endend_in_zero += 1;
        }
    }
    times_endend_in_zero
}

fn day01_part2(input: &str) -> i64 {
    let rotations = read_rotations(input);
    let mut value = 50;
    let mut times_passed_though_zero = 0;
    for rotation in rotations {
        if rotation.direction == Direction::Left {
            value = (100 - value) % 100; // Reverse value e.g. 0 -> 0, 1 -> 99, ..., 99 -> 1
        }
        value = value + rotation.number;
        times_passed_though_zero += value / 100;
        value %= 100;
        if rotation.direction == Direction::Left {
            value = (100 - value) % 100;
        }
    }
    times_passed_though_zero as i64
}

fn read_rotations(input: &str) -> Vec<Rotation> {
    input.lines().map(read_rotation).collect()
}

fn read_rotation(input: &str) -> Rotation {
    Rotation {
        direction: if input.starts_with("L") {
            Direction::Left
        } else {
            Direction::Right
        },
        number: input[1..].parse::<usize>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day01_part1(&contents);
        assert_eq!(result, 3);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day01_part1(&contents);
        assert_eq!(result, 1158);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day01_part2(&contents);
        assert_eq!(result, 6);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day01_part2(&contents);
        assert_eq!(result, 6860);
    }
}
