use std::fs;
use std::collections::HashMap;

const FILE_PATH: &str = "inputs/day2.txt";

fn get_game_id(id:&str) -> i32 {
    let id_split: Vec<&str> = id.split(" ").collect();
    let id_num = id_split[1].parse::<i32>().unwrap();
    return id_num;
}

fn check_possible_game(cubes:HashMap<&str, i32>, game:&str) -> i32 {
    let game_vec: Vec<&str> = game.split(": ").collect();
    if game_vec.len() <= 1 {
        return 0;
    }
    for set in game_vec[1].split("; ") {
        for test in set.split(", ") {
            let test_vec: Vec<&str> = test.split(" ").collect();
            // println!("{:?}", test_vec);
            if cubes[test_vec[1]] < test_vec[0].parse::<i32>().unwrap() {
                return 0;
            }
        }
    }
    return get_game_id(game_vec[0]);
}

pub fn day2_1() {
    let cubes = HashMap::from([("red", 12), ("blue", 14), ("green", 13)]);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let mut counter = 0;
    for game in contents.split("\n") {
        counter += check_possible_game(cubes.clone(), game);
    }
    println!("Output Day 2.1:\n{counter}");
}

fn get_game_power(game:&str) -> i32 {
    let game_vec: Vec<&str> = game.split(": ").collect();
    if game_vec.len() <= 1 {
        return 0;
    }
    let mut max_vals = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);
    for set in game_vec[1].split("; ") {
        for test in set.split(", ") {
            let test_vec: Vec<&str> = test.split(" ").collect();
            let num = test_vec[0].parse::<i32>().unwrap();
            if max_vals[test_vec[1]] < num {
                *max_vals.entry(test_vec[1]).or_insert(num) = num;
            }
        }
    }
    let mut multi = 1;
    for val in max_vals.values() {
        multi *= val;
    }
    return multi;
}

pub fn day2_2() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let mut counter = 0;
    for game in contents.split("\n") {
        counter += get_game_power(game);
    }
    println!("Output Day 2.2:\n{counter}");
}