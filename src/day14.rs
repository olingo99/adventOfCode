use crate::input;
//use std::collections::HashMap;


pub fn day14() -> input::Result<()> {
    let contents = input::load_day_file("day14.txt");
    let mut map : Vec<Vec<char>> = vec![];
    //let max = contents.lines().collect::<Vec<&str>>().iter().fold(vec![] |acc, x| x.split(" -> ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();//.iter().map(|x| x.split(",").collect::<Vec<&str>>()[0]).collect::<Vec<&str>>());
    let mut max_vec = vec![];
    for lines in contents.lines(){
        let segments = lines
            .split(" -> ")
            .map(|x| {
                x.split(",")
                    .map(|p| p.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        dbg!(segments);
        let data = lines.split(" -> ").collect::<Vec<&str>>();
        for elem in data.iter(){
            max_vec.push(elem.split(",").collect::<Vec<&str>>()[0].parse::<u32>().unwrap());
        }
    }
    let max = max_vec.iter().max().unwrap();
    dbg!(max);
    for line in contents.lines(){
        let data = line.split(" -> ").collect::<Vec<&str>>();
        for elem in data.iter(){
            let mut len = map.len();
            dbg!(elem);
            while len<=elem.split(",").collect::<Vec<&str>>()[1].parse::<usize>().unwrap(){
                map.push(vec!['.';*max as usize]);
                len = map.len();
            }
            // for c in elem.chars(){
            //     row.push(c);
            // }
            // map.push(row);
        }
        //dbg!(&map);
    }
    Ok(())
}
