pub fn generator(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_1(input: &[u64]) -> usize {
    input.array_windows().filter(|[a, b]| b > a).count()
}

pub fn part_2(input: &[u64]) -> usize {
    let window_sums = input
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<_>>();

    part_1(&window_sums)
}

const _RAW_TEST_INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

#[test]
fn test_part_1() {
    let input = generator(_RAW_TEST_INPUT);
    assert_eq!(part_1(&input), 7);
}

#[test]
fn test_part_2() {
    let input = generator(_RAW_TEST_INPUT);
    assert_eq!(part_2(&input), 5);
}
