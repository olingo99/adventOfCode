use crate::input;
//use std::collections::HashMap;


pub fn day7() -> input::Result<()> {

    let mut stack: Vec<u64> = vec![];
    let mut sizes = vec![];

    let contents = input::load_day_file("day7.txt");

    for block in contents.split("$"){
        let data = block.split_whitespace().collect::<Vec<&str>>();
        if data.is_empty(){continue;}
        dbg!(block);
        match data[0]{
            "cd" => {
                if data[1] == ".."{
                    let val = stack.pop();
                    sizes.push(val.unwrap());
                    let x = stack.len() as u32 -1;
                    stack[x as usize] += val.unwrap();
                }
                else{
                    stack.push(0);
                }
            },
            "ls" => {
                for line in block.lines(){
                    dbg!(line);
                    if line == " ls"{continue;}
                    //println!("{}", line);
                    if !line.contains("dir"){
                        let x = stack.len() as u32 -1;
                        //dbg!();
                        stack[x as usize]+= line.split_whitespace().collect::<Vec<&str>>()[0].parse::<u64>().unwrap();
                    }
                }
            },
            _ => {println!("Unknown command");}
        }
        
        //let mut lines = block.lines();

    }
    for i in 0..stack.len(){
        let val = stack.pop();
        sizes.push(val.unwrap());
        if stack.len() == 0 {break;}
        let x = stack.len() as u32 -1;
        stack[x as usize] += val.unwrap();
    }
    dbg!(&stack);
    dbg!(&sizes);
    // let total_used = stack[x as usize];
    let rest = 70000000-sizes.iter().max().unwrap();
    let need = 30000000 - rest;
    let y: &u64 = sizes.iter().filter(|x| **x >= need ).min().unwrap();
    dbg!(y);
    Ok(())
}
