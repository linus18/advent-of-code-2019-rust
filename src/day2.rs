#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|l| l.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let mut input2 = vec![0; input.len()];
    println!("{:?}", input);
    input2.copy_from_slice(input);
    input2[1] = 12;
    input2[2] = 2;
    println!("{:?}", input2);
    run_program(&mut input2);
    input2[0]
}

fn run_program(input: &mut [u32]) {
    let mut pos = 0;
    loop {
        let op = input[pos];
        if op == 99 {
            break;
        } else {
            let x = input[pos + 1];
            let y = input[pos + 2];
            let z = input[pos + 3];
            let f = opcode(op);
            input[z as usize] = f(input[x as usize], input[y as usize]);
        }
        pos += 4;
    }
}

fn opcode(opcode: u32) -> impl Fn(u32, u32) -> u32 {
    if opcode == 1 {
        move |x, y| x + y
    } else if opcode == 2 {
        move |x, y| x * y
    } else {
        panic!("ops: invalid opcode (was expecting 1/add or 2/multi)");
    }
}

#[cfg(test)]
pub mod tests {
    use super::run_program;

    #[test]
    fn day2_intcode() {
        let mut input = [1, 0, 0, 0, 99];
        run_program(&mut input);
        assert_eq!(input, [2, 0, 0, 0, 99]);
        //        assert_eq!(run_program(&mut [2, 4, 4, 5, 99, 0]), 9801);
        let mut input = [2, 3, 0, 3, 99];
        run_program(&mut input);
        assert_eq!(input, [2, 3, 0, 6, 99]);
        let mut input = [2, 4, 4, 5, 99, 0];
        run_program(&mut input);
        assert_eq!(input, [2, 4, 4, 5, 99, 9801]);
        let mut input = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        run_program(&mut input);
        assert_eq!(input, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
