#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|l| l.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let mut pos = 0;
    let mut input2 = vec![0; input.len()];
    println!("{:?}", input);
    input2.copy_from_slice(input);
    input2[1] = 12;
    input2[2] = 2;
    println!("{:?}", input2);
    loop {
        let op = input[pos];
        if op == 1 {
            println!("add");
            let x = input2[pos + 1];
            let y = input2[pos + 2];
            let z = input2[pos + 3];
            input2[z as usize] = input2[x as usize] + input2[y as usize];
        } else if op == 2 {
            println!("multi");
            let x = input2[pos + 1];
            let y = input2[pos + 2];
            let z = input2[pos + 3];
            input2[z as usize] = input2[x as usize] * input2[y as usize];
        } else if op == 99 {
            break;
        }
        pos += 4;
    }

    input2[0]
}
