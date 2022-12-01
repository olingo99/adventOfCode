use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    let start = Instant::now();
    let mut calories = Vec::new();
    calories.push(Vec::new());

    let mut res = Vec::new();

    let mut id = 0;
    if let Ok(lines) = read_lines("../data/CalorieCounting.txt") {
        // Consumes the iterator, returns an (Optional) String
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

    let duration = start.elapsed();
    let l = res.len();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    dbg!(res[l-1]+res[l-2]+res[l-3]);
    

}