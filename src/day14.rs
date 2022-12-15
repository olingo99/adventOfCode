use crate::input;
use std::collections::HashMap;


pub fn day14() -> input::Result<()> {
    let source = (500,0);
    let mut nb_sand = 0;
    let contents = input::load_day_file("day14.txt");
    let mut sand : HashMap<(i32,i32), char> = HashMap::new();
    let mut map : Vec<Vec<char>> = vec![];

    for lines in contents.lines(){
        let segments = lines
            .split(" -> ")
            .map(|x| {
                x.split(",")
                    .map(|p| p.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        //dbg!(&segments);



        //thx reddit (Fadi88 on github) 
        for i in 1.. segments.len(){
            let (s0, s1) = (&segments[i-1], &segments[i]);
            if s0[0] == s1[0]{
                let min = *vec![s0[1],s1[1]].iter().min().unwrap();
                let max = *vec![s0[1],s1[1]].iter().max().unwrap();
                for j in min..=max{
                    sand.insert((s0[0],j), '#');
                }
            }
            else if s0[1] == s1[1]{
                let min = *vec![s0[0],s1[0]].iter().min().unwrap();
                let max = *vec![s0[0],s1[0]].iter().max().unwrap();
                for j in min..=max{
                    sand.insert((j,s0[1]), '#');
                }
            }
        }
    }

    let mut max_y = sand.iter().map(|x| x.0.1).max().unwrap();
    max_y +=1;

    let mut abyss = false;
    while !abyss{
        let mut particule = source;
        nb_sand +=1;
        let mut is_moving = true;
        while is_moving{
            let under = sand.get(&(particule.0, particule.1+1)).unwrap_or(&'.');
            let under_left = sand.get(&(particule.0-1, particule.1+1)).unwrap_or(&'.');
            let under_right = sand.get(&(particule.0+1, particule.1+1)).unwrap_or(&'.');
            if under == &'.' && particule.1 < max_y{
                particule.1 +=1;
            }
            else if under_left == &'.' && particule.1 < max_y{
                particule.0 -=1;
                particule.1 +=1;
            }
            else if under_right == &'.' && particule.1 < max_y{
                particule.0 +=1;
                particule.1 +=1;
            }
            else if particule.1 >= max_y{
                is_moving = false;
                sand.insert(particule, 'O');
                //abyss = true;
            }
            else{
                is_moving = false;
                sand.insert(particule, 'O');
            }
        }
        if (sand.get(&(500,0)).unwrap_or(&'.') == &'O'){
            abyss = true;
        }
    }
    display(&sand);
    dbg!(nb_sand-1);
    

    Ok(())
}


fn display(sand : &HashMap<(i32,i32), char>){
    let mut min_x = 400;
    let mut max_x = 0;
    let mut min_y = 400;
    let mut max_y = 0;
    for (x,y) in sand.keys(){
        if *x < min_x{
            min_x = *x;
        }
        if *x > max_x{
            max_x = *x;
        }
        if *y < min_y{
            min_y = *y;
        }
        if *y > max_y{
            max_y = *y;
        }
    }
    dbg!(&min_x);
    dbg!(&max_x);
    dbg!(&min_y);
    dbg!(&max_y);
    for y in min_y..=max_y{
        for x in min_x..=max_x{
            print!("{}", sand.get(&(x,y)).unwrap_or(&'.'));
        }
        println!();
    }

}




    //let max = contents.lines().collect::<Vec<&str>>().iter().fold(vec![] |acc, x| x.split(" -> ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();//.iter().map(|x| x.split(",").collect::<Vec<&str>>()[0]).collect::<Vec<&str>>());
    //let mut max_vec = vec![];


        //let data = lines.split(" -> ").collect::<Vec<&str>>();
        // for elem in data.iter(){
        //     max_vec.push(elem.split(",").collect::<Vec<&str>>()[0].parse::<u32>().unwrap());
        // }


    // let max = max_vec.iter().max().unwrap();
    // dbg!(max);
    // for line in contents.lines(){
    //     let data = line.split(" -> ").collect::<Vec<&str>>();
    //     for elem in data.iter(){
    //         let mut len = map.len();
    //         dbg!(elem);
    //         while len<=elem.split(",").collect::<Vec<&str>>()[1].parse::<usize>().unwrap(){
    //             map.push(vec!['.';*max as usize]);
    //             len = map.len();
    //         }
            // for c in elem.chars(){
            //     row.push(c);
            // }
            // map.push(row);
        //}
        //dbg!(&map);
    //}