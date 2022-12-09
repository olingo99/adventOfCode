use crate::input;
use std::collections::HashSet;

pub fn day9() -> input::Result<()> {
    let contents = input::load_day_file("day9.txt");
    let mut set: HashSet<(i32,i32)>= HashSet::new();
    set.insert((0,0));
    let mut knots: Vec<(i32,i32)> = Vec::new();
    for _i in 0..10{
        knots.push((0,0));
    }
    for line in contents.lines() {
        let data = line.split(" ").collect::<Vec<&str>>();
        for _i in 0..data[1].parse::<u16>().unwrap(){
            match data[0]{
                "U" => {
                    knots[0].1+=1;
                },
                "D" => {
                    knots[0].1-=1;
                },
                "R" => {
                    knots[0].0+=1;
                },
                "L" => {
                    knots[0].0-=1;
                },
                _ => {
                    dbg!("error");
                }
            }
            dbg!("aaaaaaaaaaaa");
            for k in 1..10{
                if (knots[k - 1].0 - knots[k].0).abs() >= 2 ||
                    (knots[k - 1].1 - knots[k].1).abs() >= 2 {

                    knots[k].0 += (knots[k - 1].0 - knots[k].0).signum();
                    knots[k].1 += (knots[k - 1].1 - knots[k].1).signum();
                }
            }
            // for i in 0..knots.len()-1{
            //     // head = knots[i];
            //     // tail = knots[i+1];
                
            //     if !((knots[i].0-knots[i+1].0).abs() <= 1 && (knots[i].1-knots[i+1].1).abs() <= 1){
            //         //dbg!("moving");
            //         if ((knots[i].0-knots[i+1].0).abs() > 1 && ((knots[i].1-knots[i+1].1).abs() == 1)) || ((knots[i].1-knots[i+1].1).abs() > 1 && ((knots[i].0-knots[i+1].0).abs() == 1)){
            //             //dbg!("daig");
            //             knots[i+1].0+=if (knots[i].0-knots[i+1].0)>0 {1} else {-1};
            //             knots[i+1].1+=if (knots[i].1-knots[i+1].1)>0 {1} else {-1};
            //         }
            //         else if (knots[i].0-knots[i+1].0).abs() > 1{
            //             knots[i+1].0+=(knots[i].0-knots[i+1].0)/2;
            //         }
            //         else if (knots[i].1-knots[i+1].1).abs() > 1{
            //             knots[i+1].1+=(knots[i].1-knots[i+1].1)/2;
            //         }
            //     }
            //     //dbg!(knots[i], knots[i+1]);
            
            // }
            set.insert(knots[9]);
            //display(&knots);
            
        }
        
        
    }
    //dbg!(head, tail);
    dbg!(set.len());
    Ok(())
}


fn display(knots: &Vec<(i32, i32)>){
    for j in -20..16{
        for i in -20..16{
            if knots.contains(&(i,j)){
                let index = knots.iter().position(|&r| r == (i,j)).unwrap();
                print!("a{}",index);
            }
            else{
                print!(".");
            }
        }
        println!("");
    }
}