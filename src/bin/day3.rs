#![allow(dead_code)]

use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
    path::Path,
};

// 

fn calc_common_item_priority(data: &str) {
    let mut total_priority = 0;
    let upper_a: u32 = b'A'.into();
    let lower_a: u32 = b'a'.into();

    let lines = data.split('\n').collect::<Vec<&str>>();
    let mut lines_read = 3;

    while lines_read < lines.len() {
        let set1: HashSet<_> = lines[lines_read - 1].chars().collect();
        let set2: HashSet<_> = lines[lines_read - 2].chars().collect();
        let set3: HashSet<_> = lines[lines_read - 3].chars().collect();

        let intersection = set1
            .intersection(&set2)
            .filter(|&x| set3.contains(x))
            .collect::<Vec<_>>();

        for val in intersection {
            println!("intersection value = {}", val);
            let priority: u32 = match val.is_lowercase() {
                true => (*val as u32) - lower_a + 1,
                false => (*val as u32) - upper_a + 1 + 26,
            };
            total_priority += priority;
        }
        lines_read += 3;
    }

    println!("Total Priority: {}", total_priority);

}

fn calc_priority_sum(data: &str) {
    let mut total_priority = 0;
    let lines = data.lines();

    for line in lines {
        let mut first_half: Vec<char> = Vec::new();

        let mut common_elements: Vec<char> = Vec::new();

        let len = line.len();

        for (index, letter) in line.chars().enumerate() {
            if index < len / 2 {
                first_half.push(letter)
            } else {
                match first_half.contains(&letter) {
                    true => common_elements.push(letter),
                    false => {}
                }
            }
        }
        common_elements.dedup();

        let upper_a: u32 = b'A'.into();
        let lower_a: u32 = b'a'.into();

        for val in common_elements {
            let priority: u32 = match val.is_lowercase() {
                true => (val as u32) - lower_a + 1,
                false => (val as u32) - upper_a + 1 + 26,
            };
            println!("{} {}", val, priority);
            total_priority += priority;
        }
    }

    println!("Total Priority = {}", total_priority);
}

fn fetch_input() -> String {
    let session_key = "SESSION";
    let session_value = dotenv::var(session_key).unwrap();

    println!("Fetching input data for day 1...");

    const INPUT_URL: &str = "https://adventofcode.com/2022/day/3/input";

    let res: ureq::Response = ureq::get(INPUT_URL)
        .set("Cookie", &format!("session={}", session_value))
        .call()
        .expect("this is an error.");

    if res.status() != 200 {
        println!("Fetch failed!");
        std::process::exit(1);
    }
    res.into_string().unwrap()
}

fn main() {
    // check if file exists with inputs.
    let file_path = Path::new("./data/day3.txt");

    let file_result = File::open(file_path);

    let mut input_data: String = String::new();

    match file_result {
        Ok(mut f) => {
            println!("File found, reading input data.");
            f.read_to_string(&mut input_data)
                .expect("Could not read file.");
        }
        Err(e) => {
            // File does not exist.
            println!("File does not exist, creating and adding data. {}", e);
            let mut file = File::create(file_path).expect("File creation failed!");
            let data: String = fetch_input();

            let _written_length = file.write(data.as_bytes()).expect("Write failed!");
            println!("Done creating file and added input data.");

            input_data = data;
        }
    }

    // calc_priority_sum(&input_data);
    calc_common_item_priority(&input_data);
}
