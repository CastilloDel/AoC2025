use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day1_part1(&contents);
    println!("Day1 part 1 result: {result}");
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

fn day1_part1(input: &str) -> i64 {
    let rotations = read_rotations(input);
    let mut value = 50;
    let mut times_reached_zero = 0;
    for rotation in rotations {
        if rotation.direction == Direction::Left {
            value = (value + 100 - (rotation.number % 100)) % 100;
        } else {
            value = (value + rotation.number) % 100;
        }
        if value == 0 {
            times_reached_zero += 1;
        }
    }
    times_reached_zero
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
        let result = day1_part1(&contents);
        assert_eq!(result, 3);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day1_part1(&contents);
        assert_eq!(result, 1158);
    }
}
