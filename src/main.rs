use std::fs;

fn main() {
    let contents = fs::read_to_string("../../input").expect("Something went wrong");
    println!("result is {}", advent::day1::sum_fuel(contents.lines()));
}
