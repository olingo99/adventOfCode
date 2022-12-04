use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::io::{BufRead, BufReader};
use std::fs::File;


fn main(){
    let start = Instant::now();
    let mut score = 0;
    let reader = BufReader::new(File::open("../data/day4.txt").expect("Cannot open file.txt"));



    for line in reader.lines(){  
        let line = line.unwrap();
        let data = line.split(",").collect::<Vec<&str>>();
        let elf1 = data[0].split("-").map(|x| x.parse::<u16>().unwrap()).collect::<Vec<u16>>();
        let elf2 = data[1].split("-").map(|x| x.parse::<u16>().unwrap()).collect::<Vec<u16>>();

        let elf1range = elf1[0]..elf1[1]+1;
        let elf2range = elf2[0]..elf2[1]+1;

        if elf1range.contains(&elf2[0]) || elf2range.contains(&elf1[0]) || elf1range.contains(&elf2[1]) || elf2range.contains(&elf1[1]) {
            score += 1;
        }

    }


    dbg!(score);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

fn exo1(){
    let start = Instant::now();
    let mut score = 0;
    let reader = BufReader::new(File::open("../data/day4.txt").expect("Cannot open file.txt"));



    for line in reader.lines(){  
        let line = line.unwrap();
        let data = line.split(",").collect::<Vec<&str>>();
        let elf1 = data[0].split("-").map(|x| x.parse::<u16>().unwrap()).collect::<Vec<u16>>();
        let elf2 = data[1].split("-").map(|x| x.parse::<u16>().unwrap()).collect::<Vec<u16>>();
        // dbg!(elf1);
        // dbg!(elf2);

        // let elf1 = elf1[0]..elf1[1];
        // let elf2 = elf2[0]..elf2[1];

        // dbg!(elf2.contains(elf1));

        if elf1[0] <= elf2[0] && elf1[1] >= elf2[1] || elf2[0] <= elf1[0] && elf2[1] >= elf1[1] {
            score += 1;
        }

    }


    dbg!(score);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
