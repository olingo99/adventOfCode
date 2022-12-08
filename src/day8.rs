use crate::input;
use std::collections::HashMap;

pub fn day8() -> input::Result<()> {
    let contents = input::load_day_file("day8.txt");
    let mut bool_array : Vec<Vec<bool>>= vec![vec![]];
    let mut array : Vec<Vec<u16>> = vec![vec![]];
    for line in contents.lines(){
        dbg!(line);
        array.push(line.chars().map(|x| x.to_digit(10).unwrap() as u16).collect::<Vec<u16>>());
        for i in 0..line.len(){
            if !is_sorted(line[0..i].chars().collect::<Vec<char>>().as_slice()){
                bool_array.push(vec![true;i]);
                break;
            }
        }
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
