use std::str::FromStr;

pub enum Move {
    Up(u64),
    Down(u64),
    Forward(u64),
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(" ")
            .map(|(direction, distance)| {
                let n = distance.parse().unwrap();
                match direction {
                    "up" => Move::Up(n),
                    "down" => Move::Down(n),
                    "forward" => Move::Forward(n),
                    _ => panic!(),
                }
            })
            .ok_or(())
    }
}

pub fn generator(input: &str) -> Vec<Move> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Default)]
struct Position {
    aim: u64,
    depth: u64,
    horizontal: u64,
}

pub fn part_1(input: &[Move]) -> u64 {
    let mut pos = Position::default();

    for mov in input {
        match mov {
            Move::Up(n) => pos.depth -= n,
            Move::Down(n) => pos.depth += n,
            Move::Forward(n) => pos.horizontal += n,
        }
    }

    pos.depth * pos.horizontal
}

pub fn part_2(input: &[Move]) -> u64 {
    let mut pos = Position::default();

    for mov in input {
        match mov {
            Move::Up(n) => pos.aim -= n,
            Move::Down(n) => pos.aim += n,
            Move::Forward(n) => {
                pos.horizontal += n;
                pos.depth += n * pos.aim;
            }
        }
    }

    pos.depth * pos.horizontal
}

const _RAW_TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

#[test]
fn test_part_1() {
    let input = generator(_RAW_TEST_INPUT);
    assert_eq!(part_1(&input), 150);
}

#[test]
fn test_part_2() {
    let input = generator(_RAW_TEST_INPUT);
    assert_eq!(part_2(&input), 900);
}
