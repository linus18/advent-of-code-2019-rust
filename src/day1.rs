pub fn sum_fuel<'a, I>(lines: I) -> u32
where
    I: Iterator<Item = &'a str>,
{
    let mut acc = 0;
    for line in lines {
        let x: u32 = line.parse().unwrap();
        acc += x / 3 - 2;
    }
    acc
}

#[test]
fn part1() {
    let result = sum_fuel("12".lines());
    assert!(result == 2, "Should be 2 but was {}", result);
    let result = sum_fuel("14".lines());
    assert!(result == 2, "Should be 2 but was {}", result);
    let result = sum_fuel("1969".lines());
    assert!(result == 654, "Should be 654 but was {}", result);
    let result = sum_fuel("100756".lines());
    assert!(result == 33583, "Should be 33583 but was {}", result);
}
