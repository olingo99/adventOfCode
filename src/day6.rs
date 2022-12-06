use crate::input;
use std::collections::HashSet;

pub fn day6() -> input::Result<()> {
    let contents = input::load_day_file("day6.txt");
    let window_size = 14;
    
    for (i,window) in contents.chars().collect::<Vec<char>>().windows(window_size).enumerate() {
        let mut set = HashSet::new();
        for elem in window {
            set.insert(elem);
        }
        if set.len() == window_size {
            dbg!(i+window_size);
            break;
        }
    }


    //dbg!(res);
    Ok(())
}
