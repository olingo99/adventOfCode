use crate::input;
use std::collections::HashMap;

pub fn day8() -> input::Result<()> {
    let contents = input::load_day_file("day8.txt");
    let mut bool_array : Vec<Vec<bool>>= vec![vec![]];
    let mut array : Vec<Vec<u16>> = vec![vec![]];
    for line in contents.lines(){
        dbg!(line);
        array.push(line.chars().map(|x| x.to_digit(10).unwrap() as u16).collect::<Vec<u16>>());
        let chars = line.chars().collect::<Vec<char>>();
        let p_char = chars[0];
        for k in 1..chars.len()+1{
            for (i,window) in line.chars().collect::<Vec<char>>().windows(k).enumerate() {

            }
        }

        // for i in 1..chars.len(){
        //     //if chars[i].to_digit(10).unwrap()<p_char.to_digit(10).unwrap(){
        //     if{
        //         bool_array.push(vec![true;i]);
        //         let x = bool_array.len()-1;
        //         bool_array[x].append(& mut vec![false;chars.len()-i]);
        //         break;
        //     }
        // }
    }
    dbg!(array);
    dbg!(bool_array);
    Ok(())
}

fn is_sorted<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}
