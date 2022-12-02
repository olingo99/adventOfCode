// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;
// use std::collections::HashMap;
// use std::time::{Duration, Instant};

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }


// fn main() {
//     let mut score : u32 = 0;
//     let mut enemy : u32 = 0;
//     let mut player : u32 = 0;
//     let mut split;
//     if let Ok(lines) = read_lines("../data/rpc.txt") {
//         // Consumes the iterator, returns an (Optional) String
//         for line in lines {
//             if let Ok(ip) = line {
//                 split = ip.split(" ").collect::<Vec<&str>>();
//                 dbg!(split);
//                 // enemy = split[0].parse::<u32>().unwrap();
//                 // player = split[1].parse::<u32>().unwrap();
//                 // score += player + player;
//             }
//         }
//     }

//     dbg!(2%3);
    

// }


use std::io::{BufRead, BufReader};
use std::fs::File;
fn main(){
    let mut score : u32 = 0;
    let mut enemy : u32 = 0;
    let mut player : u32 = 0;
    let reader = BufReader::new(File::open("../data/rpc.txt").expect("Cannot open file.txt"));

    for line in reader.lines() {
        let words = line.as_ref().unwrap().split(" ").collect::<Vec<&str>>();
        enemy = translate(words[0]);
        player = translate(words[1]);
        dbg!(enemy);
        dbg!(player);
    }
}

fn translate(word: &str ) -> u32{
    match word {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}
