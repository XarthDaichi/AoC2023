use std::fs;

const FILE_PATH: &str = "inputs/day3.txt";

pub fn day3_1() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let mut engine_mat = Vec::new();
    engine_mat.push(Vec::new());
    engine_mat.pop();
    contents.split("\n").for_each(|el| engine_mat.push(el.chars().collect()));
    engine_mat.pop();

    let lines_len = engine_mat[0].len();

    // println!("{:?}", engine_mat);
    let mut counter = 0;
    let mut valid = false;
    let mut started_num = engine_mat[0][0].is_numeric();
    let mut ended_num = false;
    let mut num_starting_pos:usize = 0;
    let mut num_ending_pos:usize = 0;

    for i in 0..engine_mat.len() {
        valid = false;
        started_num = false;
        ended_num = false;
        // println!("{:?}", engine_mat[i]);
        for j in 0..lines_len {
            if !started_num {
                valid = false;
            }
            if engine_mat[i][j].is_numeric() {
                if !started_num {
                    // print!("{}", engine_mat[i][j]);
                    started_num = true;
                    ended_num = false;
                    num_starting_pos = j;
                }
                if i > 0 {
                    if j > 0 {
                        if !valid && !engine_mat[i-1][j-1].is_numeric() && engine_mat[i-1][j-1] != '.' {
                            valid = true;
                        }
                    }
                    if !valid && !engine_mat[i-1][j].is_numeric() && engine_mat[i-1][j] != '.' {
                        valid = true;
                    }
                    if j < lines_len - 1 {
                        if !valid && !engine_mat[i-1][j+1].is_numeric() && engine_mat[i-1][j+1] != '.' {
                            valid = true;
                        }
                    }
                }
                if i < engine_mat.len() - 1{
                    if j > 0 {
                        if !valid && !engine_mat[i+1][j-1].is_numeric() && engine_mat[i+1][j-1] != '.' {
                            valid = true;
                        }
                    }
                    if !valid && !engine_mat[i+1][j].is_numeric() && engine_mat[i+1][j] != '.' {
                        valid = true;
                    }
                    if j < lines_len - 1 {
                        if !valid && !engine_mat[i+1][j+1].is_numeric() && engine_mat[i+1][j+1] != '.' {
                            valid = true;
                        }
                    }
                }
                if j > 0 {
                    if !valid && !engine_mat[i][j-1].is_numeric() && engine_mat[i][j-1] != '.' {
                        valid = true;
                    }
                }
                if j < lines_len - 1 {
                    if !valid && !engine_mat[i][j+1].is_numeric() && engine_mat[i][j+1] != '.' {
                        valid = true;
                    }
                }
                // print!("{}", engine_mat[i][j]);
                if j == lines_len - 1 || (j < lines_len - 1 && !engine_mat[i][j+1].is_numeric()) {
                    // print!("{}\n", engine_mat[i][j]);
                    ended_num = true;
                    started_num = false;
                    num_ending_pos = j;
                }
                if ended_num && valid {
                    counter += engine_mat[i][num_starting_pos..=num_ending_pos]
                                    .into_iter()
                                    .map(|c| c.to_string())
                                    .collect::<Vec<String>>()
                                    .join("")
                                    .parse::<i32>().unwrap();
                    // println!("start = {num_starting_pos}, end = {num_ending_pos}, num = {:?} | i = {i}, j = {j}, char = {} | {counter}", &engine_mat[i][num_starting_pos..=num_ending_pos], engine_mat[i][j]);
                    ended_num = false;
                }
            }
        }
    }

    println!("Output Day 3.1:\n{counter}");
}

fn get_matrix_number(line:Vec<char>, pos:usize) -> i32 {
    let mut min = 0;
    let mut max = 1000;
    let before_rev:Vec<&char> = line[..=pos].into_iter().rev().collect();
    // println!("before = {:?}", before_rev);
    if before_rev.len() == 1 {
        min = pos;
    }
    for i in 0..before_rev.len() {
        if i == before_rev.len() - 1 || (i < before_rev.len() - 1 && !before_rev[i+1].is_numeric()) {
            min = pos - i;
            break;
        } 
    }
    let after_vec:Vec<&char> = line[pos..].into_iter().collect();
    // println!("after = {:?}", after_vec);
    let after_len = after_vec.len();

    if after_len == 1{
        max = pos;
    }
    for i in 0..after_len {
        if i == after_len - 1 || (i < after_len - 1 && !after_vec[i+1].is_numeric()) {
            max = pos + i;
            break;
        }
    }

    return line[min..=max].into_iter().map(|c| c.to_string()).collect::<Vec<String>>().join("").parse::<i32>().unwrap();
}

pub fn day3_2() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let mut engine_mat = Vec::new();
    engine_mat.push(Vec::new());
    engine_mat.pop();
    contents.split("\n").for_each(|el| engine_mat.push(el.chars().collect()));
    engine_mat.pop();

    let lines_len = engine_mat[0].len();

    // println!("{:?}", engine_mat);
    let mut counter = 0;
    let mut parts_vec = Vec::new();
    let mut multiplier = 1;
    let mut top_left = false;
    let mut top_mid = false;
    let mut bot_left = false;
    let mut bot_mid = false;

    for i in 0..engine_mat.len() {
        for j in 0..lines_len {
            if engine_mat[i][j] == '*' {
                parts_vec.clear();
                multiplier = 1;
                top_left = false;
                top_mid = false;
                bot_left = false;
                bot_mid = false;
                if i > 0 {
                    if j > 0 {
                        if engine_mat[i-1][j-1].is_numeric() {
                            parts_vec.push(get_matrix_number(engine_mat[i-1].clone(), j-1));
                            top_left = true;
                        }
                    }
                    if engine_mat[i-1][j].is_numeric() {
                        if !top_left {
                            parts_vec.push(get_matrix_number(engine_mat[i-1].clone(), j));
                        }
                        top_mid = true;
                    }
                    if engine_mat[i-1][j+1].is_numeric() {
                        if !top_mid {
                            parts_vec.push(get_matrix_number(engine_mat[i-1].clone(), j+1));
                        }
                    }
                }
                if i < engine_mat.len() - 1 {
                    if j > 0 {
                        if engine_mat[i+1][j-1].is_numeric() {
                            parts_vec.push(get_matrix_number(engine_mat[i+1].clone(), j-1));
                            bot_left = true;
                        }
                    }
                    if engine_mat[i+1][j].is_numeric() {
                        if !bot_left {
                            parts_vec.push(get_matrix_number(engine_mat[i+1].clone(), j));
                        }
                        bot_mid = true;
                    }
                    if j < lines_len - 1 {
                        if engine_mat[i+1][j+1].is_numeric() {
                            if !bot_mid {
                                parts_vec.push(get_matrix_number(engine_mat[i+1].clone(), j+1));
                            }
                        }
                    }
                }
                if j > 0 {
                    if engine_mat[i][j-1].is_numeric() {
                        parts_vec.push(get_matrix_number(engine_mat[i].clone(), j-1));
                    }
                }
                if j < lines_len - 1 {
                    if engine_mat[i][j+1].is_numeric() {
                        parts_vec.push(get_matrix_number(engine_mat[i].clone(), j+1));
                    }
                }
                // println!("{:?}", parts_vec);
                if parts_vec.len() > 1 {
                    parts_vec.clone().into_iter().for_each(|elem| multiplier *= elem);
                    counter += multiplier;
                }
            }
        }
    }

    println!("Output Day 3.2:\n{counter}");
}