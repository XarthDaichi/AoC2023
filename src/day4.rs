use std::fs;

const FILE_PATH: &str = "inputs/day4.txt";

fn binary_search(x:i32, list:Vec<i32>) -> bool {
    let mut min = 0;
    let mut max = list.len() as i32 - 1;
    while min <= max {
        let mid = ((max - min) / 2) + min;
        let mid_index = mid as usize;
        let target_value = list[mid_index];
        if x == target_value {
            return true;
        }

        if x > target_value {
            min = mid + 1;
        }

        if x < target_value {
            max = mid - 1;
        }
    }
    return false;
}

fn points_calculator(x:i32) -> i32 {
    if x == 0 {
        return 0;
    }
    let mut points = 1;
    let mut times = x-1;
    while times > 0 {
        points *= 2;
        times -= 1;
    }

    return points;
}

pub fn day4_1() {
    let mut counter = 0;
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    // println!("{}", contents);
    
    for game in contents.split("\n") {
        let cards_vec = game.split(": ").collect::<Vec<&str>>();
        if cards_vec.len() > 1 {
            let line_vec = cards_vec[1].split(" | ").collect::<Vec<&str>>();
            let mut scratcher = line_vec[0].split(" ").into_iter().filter(|elem| !elem.is_empty()).map(|elem| elem.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            scratcher.sort_unstable();
            let mut possibilities = line_vec[1].split(" ").into_iter().filter(|elem| !elem.is_empty()).map(|elem| elem.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            possibilities.sort_unstable();
            // println!("{:?}", scratcher.clone().into_iter().filter(|elem| binary_search(*elem, possibilities.clone())).collect::<Vec<i32>>());
            counter += points_calculator(scratcher.into_iter().filter(|elem| binary_search(*elem, possibilities.clone())).collect::<Vec<i32>>().len() as i32);
            // println!("count:{counter}")
        }
    }

    println!("Output Day 4.1:\n{counter}");

}

pub fn day4_2() {
    let mut counter = 0;

    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let mut copies = vec![1; contents.clone().split("\n").collect::<Vec<&str>>().len()-1];

    for (i, game) in contents.split("\n").enumerate() {
        let cards_vec = game.split(": ").collect::<Vec<&str>>();
        if cards_vec.len() > 1 {
            let line_vec = cards_vec[1].split(" | ").collect::<Vec<&str>>();
            let mut scratcher = line_vec[0].split(" ").into_iter().filter(|elem| !elem.is_empty()).map(|elem| elem.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            scratcher.sort_unstable();
            let mut possibilities = line_vec[1].split(" ").into_iter().filter(|elem| !elem.is_empty()).map(|elem| elem.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            possibilities.sort_unstable();
            let wins = scratcher.clone().into_iter().filter(|elem| binary_search(*elem, possibilities.clone())).collect::<Vec<i32>>().len();
            // println!("{:?}", scratcher.clone().into_iter().filter(|elem| binary_search(*elem, possibilities.clone())).collect::<Vec<i32>>());
            for j in 1..=wins {
                copies[i + j] += copies[i];
            }
            counter += copies[i];
            // println!("copies: {:?} | count:{counter}", copies);
        }
    }

    println!("Output Day 4.2:\n{counter}");
}