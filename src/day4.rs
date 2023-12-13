use std::fs;

const FILE_PATH: &str = "inputs/day4ex.txt";

pub fn day4_1() {
    let mut counter = 0;
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    
    println!("Output Day 4.1:\n{counter}");

}

pub fn day4_2() {
    let mut counter = 0;
    println!("Output Day 4.2:\n{counter}");
}