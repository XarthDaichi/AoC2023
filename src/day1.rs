use std::fs;
 
const FILE_PATH: &str = "inputs/day1.txt";

pub fn day1_1() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");


    let bytes = contents.as_bytes();
    let mut counter = 0;
    let mut opening_num = 0;
    let mut closing_num = 0;
    for (_i, &item) in bytes.iter().enumerate() {
        if item.is_ascii_digit() {
            if opening_num == 0 {
                opening_num = i32::from(item - b'0');
                closing_num = opening_num;
                // println!("opening_num: {opening_num}");
            } else {
                closing_num = i32::from(item - b'0');
                // println!("closing_num: {closing_num}");
            }
        }
        if item == b'\n' {
            counter += opening_num *10 + closing_num;
            // println!("counter: {counter}");
            opening_num = 0;
            closing_num = 0;
        }
    }
    if opening_num != 0 {
        counter += opening_num *10 + closing_num;
    }
    println!("Output Day 1.1:\n{counter}");
}

pub fn day1_2() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read file");

    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    // let contents = "onetwothree";
    // this is how slicing works
    // let test = "one";
    // let test_num = 0;

    // assert_eq!(&test[test_num..test_num+3], numbers[0]);
    
    let mut counter = 0;
    let mut opening_num = 0;
    let mut closing_num = 0;

    for (i, ch) in contents.chars().enumerate() {
        if ch.is_numeric() {
            if opening_num == 0 {
                opening_num = ch as i32 - 0x30;
                closing_num = opening_num;
            } else {
                closing_num = ch as i32 - 0x30;
            }
            continue;
        }
        if ch == 'o' {
            if i+3 < contents.len() && &contents[i..i+3] == numbers[0] {
                if opening_num == 0 {
                    opening_num = 1;
                    closing_num = 1;
                } else {
                    closing_num = 1;
                }
            }
            continue;
        }
        if ch == 't' {
            if i+3 < contents.len() && &contents[i..i+3] == numbers[1] {
                if opening_num == 0 {
                    opening_num = 2;
                    closing_num = 2;
                } else {
                    closing_num = 2;
                }
                continue;
            }
            if i+5 < contents.len() && &contents[i..i+5] == numbers[2] {
                if opening_num == 0 {
                    opening_num = 3;
                    closing_num = 3;
                } else {
                    closing_num = 3;
                }
                continue;
            }
        }
        if ch == 'f' {
            if i+4 < contents.len() && &contents[i..i+4] == numbers[3] {
                if opening_num == 0 {
                    opening_num = 4;
                    closing_num = 4;
                } else {
                    closing_num = 4;
                }
                continue;
            }
            if i+4 < contents.len() && &contents[i..i+4] == numbers[4] {
                if opening_num == 0 {
                    opening_num = 5;
                    closing_num = 5;
                } else {
                    closing_num = 5;
                }
                continue;
            }
        }
        if ch == 's' {
            if i+3 < contents.len() && &contents[i..i+3] == numbers[5] {
                if opening_num == 0 {
                    opening_num = 6;
                    closing_num = 6;
                } else {
                    closing_num = 6;
                }
                continue;
            }
            if i+5 < contents.len() && &contents[i..i+5] == numbers[6] {
                if opening_num == 0 {
                    opening_num = 7;
                    closing_num = 7;
                } else {
                    closing_num = 7;
                }
                continue;
            }
        }
        if ch == 'e' {
            if i+5 < contents.len() && &contents[i..i+5] == numbers[7] {
                if opening_num == 0 {
                    opening_num = 8;
                    closing_num = 8;
                } else {
                    closing_num = 8;
                }
                continue;
            }
        }
        if ch == 'n' {
            if i+4 < contents.len() && &contents[i..i+4] == numbers[8] {
                if opening_num == 0 {
                    opening_num = 9;
                    closing_num = 9;
                } else {
                    closing_num = 9;
                }
                continue;
            }
        }
        if ch == '\n' {
            // println!("temp: {opening_num}{closing_num}");
            counter += opening_num*10 + closing_num;
            opening_num = 0;
            closing_num = 0;
        }
    }

    if opening_num != 0 {
        counter += opening_num *10 + closing_num;
    }

    println!("Output Day 1.2:\n{counter}");
}