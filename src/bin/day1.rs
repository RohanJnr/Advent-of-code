use std::{
    fs::File,
    io::{Read, Write},
    path::Path
};
use std::time::Instant;


fn calc_top_three(data: &str) {
    // Wanted to have independent solutions for each part, hence code is repeated.
    let calories = data.split("\n\n").map(|item| {
        item.split('\n')
        .filter_map(|c| c.parse::<i32>().ok())
        .sum::<i32>()
    });

    let mut calories: Vec<i32> = calories.collect();
    calories.sort_unstable_by(|a, b| b.cmp(a));

    let max = calories.iter().max().unwrap().to_owned();

    let result: i32 = calories.iter().take(3).sum();
    
    println!("max = {}", max);
    println!("top 3 = {:?}", result);

}

fn calc_highest_calories(data: &str) {
    let parts: Vec<&str> = data.lines().collect();

    let mut highest = 0;
    let mut current = 0;

    for part in parts {
        match part.is_empty() {
            true => {
                if current > highest {
                    highest = current;
                }

                current = 0;
            }
            false => current += part.parse::<i32>().unwrap(),
        }
    }
    println!("Highest = {}", highest);
}

fn fetch_input() -> String {
    let session_key = "SESSION";
    let session_value = dotenv::var(session_key).unwrap();

    println!("Fetching input data for day 1...");

    const INPUT_URL: &str = "https://adventofcode.com/2022/day/1/input";

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
    let file_path = Path::new("./data/day1.txt");

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
    let start = Instant::now();
    calc_highest_calories(&input_data);
    let elapsed = start.elapsed();
    calc_top_three(&input_data);
    println!("Time elapsed in expensive_function() is: {:?}", elapsed);
}
