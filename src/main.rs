use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    let mut calories = Vec::new();
    calories.push(Vec::new());

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

    //let mut res: Vec<u32> = Vec::new();
    let mut res:u32 = 0;
    for elf in calories{
        res = elf.iter().sum();
    }   
    dbg!(res);
    

}