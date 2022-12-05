use std::io::{BufRead, BufReader};
use std::fs::File;

use crate::input;


pub fn day3() -> input::Result<()> {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut score = 0;

    let mut temp_group = Vec::new();
    let mut groups = Vec::new();



    let reader = BufReader::new(File::open("../data/day3.txt").expect("Cannot open file.txt"));



    for line in reader.lines(){
        temp_group.push(line.unwrap());
        
        if temp_group.len() == 3 {
            groups.push(temp_group);
            temp_group = Vec::new();
        }    
    }
    for group in groups {
        let line1 = group[0].clone();
        let line2 = group[1].clone();
        let line3 = group[2].clone();
        for c in line1.chars() {
            if line2.contains(c) && line3.contains(c) {
                score+= alphabet.find(c).unwrap()+1;
                break;
            }
        }
    }

    dbg!(score);

    Ok(())
}

fn exo1(){
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut score = 0;

    let reader = BufReader::new(File::open("../data/rucksack.txt").expect("Cannot open file.txt"));

    for line in reader.lines() {
        let data = line.unwrap();
        let first_half = data.split_at(data.len()/2).0;
        let second_half = data.split_at(data.len()/2).1;
        // dbg!(first_half);
        // dbg!(second_half);
        for c in first_half.chars() {
            //dbg!(c);
            if second_half.contains(c) {
                // dbg!(c);
                score+= alphabet.find(c).unwrap()+1;
                break;
            }
        }

    }

    dbg!(score);
}
