
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::input;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn day1() -> input::Result<()>{
    let mut calories = Vec::new();
    calories.push(Vec::new());

    let mut res = Vec::new();

    let mut id = 0;
    if let Ok(lines) = read_lines("../data/day1.txt") {
        for line in lines {
            if let Ok(ip) = line {

                if ip.is_empty() {
                    calories.push(Vec::new());
                    id+=1;
                }
                else{
                    calories[id].push(ip.parse::<u32>().unwrap());
                }
            }
        }
    }

    res = calories.iter().map(|x| x.iter().sum()).collect::<Vec<u32>>();

    res.sort();

    let l = res.len();
    dbg!(res[l-1]+res[l-2]+res[l-3]);
    Ok(())
}

    