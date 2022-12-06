use crate::input;
//use std::collections::HashMap;

pub fn day6() -> input::Result<()> {
    let contents = input::load_day_file("day6.txt");
    
    let mut circular_buffer = Vec::new();
    let mut current_position = 0;

    for (i,cha) in contents.chars().enumerate() {
        // if circular_buffer.len()>=3 && cha not in circular_buffer {
        //     let res = 
        //     break;
        // }
        circular_buffer.insert(current_position, cha);
        current_position += 1;
        dbg!(i);
        dbg!(cha);
    }


    Ok(())
}
