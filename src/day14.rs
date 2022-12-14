use crate::input;
//use std::collections::HashMap;

pub fn day14() -> input::Result<()> {
    
    let contents = input::load_day_file("day14.txt");
    let mut map : Vec<Vec<char>> = vec![];
    for line in contents.lines(){
        let data = line.split(" -> ").collect::<Vec<&str>>();
        dbg!(data);
    }
    Ok(())
}
