#![allow(dead_code)]

use std::{
    fs::File,
    io::{Read, Write},
    iter::Iterator,
    path::Path,
};

use itertools::Itertools;

// A - Rock
// B - Paper
// C - Scissors

// X - Rock
// Y - Paper
// Z - Scissors

fn calc_score_1_fancy(data: &str) {
    let c = data
        .split('\n')
        .map(|item| {
            let (opponent, player) = item
                .split_whitespace()
                .map(
                    |game_input| match game_input.strip_suffix('\n').unwrap_or(game_input) {
                        "A" | "X" => "rock",
                        "B" | "Y" => "paper",
                        "C" | "Z" => "scissor",
                        _ => "nothing",
                    },
                )
                .collect_tuple()
                .unwrap();

            match (opponent, player) {
                ("rock", "paper") => 2 + 6,
                ("rock", "scissor") => 3,
                ("rock", "rock") => 1 + 3,
                ("paper", "paper") => 2 + 3,
                ("paper", "scissor") => 3 + 6,
                ("paper", "rock") => 1,
                ("scissor", "paper") => 1,
                ("scissor", "scissor") => 3 + 3,
                ("scissor", "rock") => 1 + 6,
                _ => 0,
            }
        })
        .sum::<i32>();

    println!("Score = {}", c);
}


fn calc_score_2(data: &str) {
    let mut score = 0;
    let lines = data.lines();

    for line in lines {
        let inputs: (&str, &str) = line.split_once(' ').unwrap();
        let score_per_round = match (inputs.0, inputs.1) {
            ("A", "X") => 3,
            ("A", "Y") => 1 + 3,
            ("A", "Z") => 2 + 6,
            ("B", "X") => 1,
            ("B", "Y") => 2 + 3,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 2,
            ("C", "Y") => 3 + 3,
            ("C", "Z") => 1 + 6,
            _ => 0,
        };

        score += score_per_round;

    }

    println!("Score: {}", score);


}

fn calc_score_1_simple(data: &str) {
    let mut score = 0;
    let lines = data.lines();

    for line in lines {
        let inputs: (&str, &str) = line.split_whitespace().map(|game_input| {
            match game_input {
                "A" | "X" => "rock",
                "B" | "Y" => "paper",
                "C" | "Z" => "scissor",
                _ => "nothing",
            }
        }).collect_tuple().unwrap();

        // println!("oppoent = {}, user = {}", inputs.0, inputs.1);

        let score_per_round = match (inputs.0, inputs.1) {
            ("rock", "paper") => 2 + 6,
            ("rock", "scissor") => 3,
            ("rock", "rock") => 1 + 3,
            ("paper", "paper") => 2 + 3,
            ("paper", "scissor") => 3 + 6,
            ("paper", "rock") => 1,
            ("scissor", "paper") => 2,
            ("scissor", "scissor") => 3 + 3,
            ("scissor", "rock") => 1 + 6,
            _ => 0,
        };

        score += score_per_round;
    }
    println!("Score: {}", score);
}

fn fetch_input() -> String {
    let session_key = "SESSION";
    let session_value = dotenv::var(session_key).unwrap();

    println!("Fetching input data for day 1...");

    const INPUT_URL: &str = "https://adventofcode.com/2022/day/2/input";

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
    let file_path = Path::new("./data/day2.txt");

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

            let _write_length = file.write(data.as_bytes()).expect("Write failed!");
            println!("Done creating file and added input data.");

            input_data = data;
        }
    }
    calc_score_1_simple(&input_data);
    calc_score_2(&input_data);
}
