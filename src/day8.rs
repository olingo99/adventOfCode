use crate::input;
use std::collections::HashMap;

pub fn day8() -> input::Result<()> {
    let contents = input::load_day_file("day8.txt");
    let mut bool_array : Vec<Vec<bool>>= Vec::new();
    let mut array : Vec<Vec<u16>> = Vec::new();
    let mut views = Vec::new();
    for line in contents.lines(){
        array.push(line.chars().map(|x| x.to_digit(10).unwrap() as u16).collect::<Vec<u16>>());
    }
    for i in 0..array.len(){
        for j in 0..array[i].len(){
            let mut temp = 1;
            let mut array_top = Vec::new();
            let mut array_bot = Vec::new();
            let mut array_left = array[i][0..j as usize].to_vec();
            let mut array_right = array[i][j as usize+1..array[0].len()].to_vec();
            array_left.reverse();
            for k in 0..i{
                array_top.push(array[k as usize][j as usize]);
            }
            for k in i+1..array[0].len(){
                array_bot.push(array[k as usize][j as usize]);
            }
            array_top.reverse();
            for gen_array in vec![array_bot, array_top, array_left, array_right]{
                let mut temp_array = gen_array.clone();
                if i == 3 && j==2{
                    dbg!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
                }
                //dbg!(&temp_array);
                
                for (a,elem) in temp_array.iter().enumerate(){

                    //dbg!(a,elem);

                    if *elem>=array[i as usize][j as usize] || a==temp_array.len()-1{
                        //dbg!(a);
                        temp *= a as u32+1;
                        break;
                    }
                }
            }
            dbg!(temp);
            views.push(temp);
        }
    }
    dbg!(views.iter().max().unwrap());
    Ok(())
}



pub fn day8_exo1() -> input::Result<()> {
    let contents = input::load_day_file("day8.txt");
    let mut bool_array : Vec<Vec<bool>>= Vec::new();
    let mut array : Vec<Vec<u16>> = Vec::new();
    for line in contents.lines(){
        //dbg!(line);
        array.push(line.chars().map(|x| x.to_digit(10).unwrap() as u16).collect::<Vec<u16>>());
        let chars = line.chars().map(|x| x.to_digit(10).unwrap() as u16).collect::<Vec<u16>>();
        let p_char = chars[0];
        let mut temp_vec = vec![];
        for k in 0..chars.len(){
            // dbg!(chars[k]);
            // dbg!(&chars[0..k+1]);
            // dbg!(chars[k] == *chars[0..k+1].iter().max().unwrap());
            // dbg!(chars[0..k+1].iter().filter(|&n| *n == chars[k]).count() == 1);
            temp_vec.push((chars[k] == *chars[0..k+1].iter().max().unwrap() && chars[0..k+1].iter().filter(|&n| *n == chars[k]).count() == 1)||(chars[k] == *chars[k..chars.len()].iter().max().unwrap() && chars[k..chars.len()].iter().filter(|&n| *n == chars[k]).count() == 1));
        }
        bool_array.push(temp_vec);
    }
    for i in 0..array[0].len(){
        let mut col = Vec::new();
        for row in array.clone(){
            col.push(row[i]);
        }
        for k in 0..col.len(){
            //dbg!(col[k] == *col[0..k+1].iter().max().unwrap() && col[0..k+1].iter().filter(|&n| *n == col[k]).count() == 1);
            //dbg!((col[k] == *col[k..col.len()].iter().max().unwrap() && col[k..col.len()].iter().filter(|&n| *n == col[k]).count() == 1));
            bool_array[k][i] =bool_array[k][i]||(col[k] == *col[0..k+1].iter().max().unwrap() && col[0..k+1].iter().filter(|&n| *n == col[k]).count() == 1)||(col[k] == *col[k..col.len()].iter().max().unwrap() && col[k..col.len()].iter().filter(|&n| *n == col[k]).count() == 1);
        }
    }
    let res : usize = bool_array.iter().map(|x| x.iter().filter(|&n| *n == true).count()).sum();
    dbg!(res);
    // dbg!(&array);
    // dbg!(&bool_array);
    Ok(())
}
