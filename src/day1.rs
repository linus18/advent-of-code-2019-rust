#[aoc_generator(day1)]
fn generator_input(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse::<u32>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn sum_fuel(lines: &[u32]) -> u32
{
    let mut acc = 0;
    for line in lines {
        acc += line / 3 - 2;
    }
    acc
}

#[test]
fn part1() {
    let result = sum_fuel(&[12]);
    assert!(result == 2, "Should be 2 but was {}", result);
    let result = sum_fuel(&[14]);
    assert!(result == 2, "Should be 2 but was {}", result);
    let result = sum_fuel(&[1969]);
    assert!(result == 654, "Should be 654 but was {}", result);
    let result = sum_fuel(&[100756]);
    assert!(result == 33583, "Should be 33583 but was {}", result);
}
