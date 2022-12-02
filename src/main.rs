// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;
use std::collections::HashMap;
use std::time::{Duration, Instant};


use std::io::{BufRead, BufReader};
use std::fs::File;
fn main(){
    let start = Instant::now();
    let mut score : u32 = 0;
    let reader = BufReader::new(File::open("../data/rpc.txt").expect("Cannot open file.txt"));
    let map : HashMap<String, u32> = HashMap::from([
        ("A X".to_string(), 3),
        ("A Y".to_string(), 4),
        ("A Z".to_string(), 8),
        ("B X".to_string(), 1),
        ("B Y".to_string(), 5),
        ("B Z".to_string(), 9),
        ("C X".to_string(), 2),
        ("C Y".to_string(), 6),
        ("C Z".to_string(), 7),
    ]);
    for line in reader.lines() {

        score += map.get(&line.unwrap()).unwrap();
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    dbg!(score);
}

fn translate(word: &str ) -> u32{
    match word {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0,
    }
}

fn exo1(){
    let map : HashMap<String, u32> = HashMap::from([
        ("A X".to_string(), 4),
        ("A Y".to_string(), 8),
        ("A Z".to_string(), 3),
        ("B X".to_string(), 1),
        ("B Y".to_string(), 5),
        ("B Z".to_string(), 9),
        ("C X".to_string(), 7),
        ("C Y".to_string(), 2),
        ("C Z".to_string(), 6),
    ]);
    let mut score : u32 = 0;
    // let mut enemy : u32 = 0;
    // let mut player : u32 = 0;
    let reader = BufReader::new(File::open("../data/rpc.txt").expect("Cannot open file.txt"));

    for line in reader.lines() {
        //score += map.get(line.unwrap()).unwrap();
        //dbg!(line.unwrap());
        score += map.get(&line.unwrap()).unwrap();
    }
    dbg!(score);
}
