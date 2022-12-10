use crate::input;
use std::collections::HashMap;

pub fn day10() -> input::Result<()> {
    let contents = input::load_day_file("day10.txt");
    let mut vec = Vec::new();
    let mut display = vec![[".";40];6];
    let mut acc: i32 = 1;
    let mut cycle:i32 = 0;
    for line in contents.lines(){
        let data = line.split(" ").collect::<Vec<&str>>();
        match data[0]{
            "noop" => {
                if (acc-1..acc+2).contains(&(cycle%40)){
                    dbg!(cycle/40);
                    dbg!(cycle%40);
                    display[((cycle)/40) as usize][((cycle)%40) as usize] = "X";
                }
                cycle+=1;
                
                if (cycle -20)%40 == 0{
                    //dbg!(acc);
                    //dbg!(cycle);
                    vec.push(acc*cycle);
                }
            },
            "addx" => {
                if (acc-1..acc+2).contains(&(cycle%40)){
                    dbg!(cycle/40);
                    dbg!(cycle%40);
                    display[((cycle)/40) as usize][((cycle)%40) as usize] = "X";
                }
                cycle+=1;
                 if (cycle -20)%40 == 0{
                    //dbg!(acc);
                    //dbg!(cycle);
                    vec.push(acc*cycle);
                }
                if (acc-1..acc+2).contains(&(cycle%40)){
                    dbg!(cycle/40);
                    dbg!(cycle%40);
                    display[((cycle)/40) as usize][((cycle)%40) as usize] = "X";
                }
                cycle+=1;
                
                if (cycle -20)%40 == 0{
                    //dbg!("aaaa");
                    //dbg!(acc);
                    //dbg!(cycle);
                    vec.push(acc*cycle);
                }
                acc += data[1].parse::<i32>().unwrap();
            },
            _ => {panic!("Unknown instruction {}", data[0]);}
        }
        // if (acc-1..acc+2).contains(&(cycle%40)){
        //     dbg!(cycle/40);
        //     dbg!(cycle%40);
        //     display[((cycle)/40) as usize][((cycle)%40) as usize] = "X";
        // }

    }
    dbg!(&vec);
    dbg!(vec.iter().sum::<i32>());
    for elem in display{
        println!("{:?}", elem.join(""));
    }
    Ok(())
}


// #[derive(Debug)]
// enum Instruction {
//     Noop,
//     Addx(isize),
// }

// fn run(instruction: &Instruction, register: &mut Vec<isize>) {
//     let &current = register.last().unwrap();
//     match instruction {
//         Instruction::Noop => register.push(current),
//         Instruction::Addx(x) => {
//             register.push(current);
//             register.push(current + *x);
//         }
//     }
// }

// const SIZE: usize = 40;
// pub fn day10() -> input::Result<()> {
//     let mut register = vec![1];
//     include_str!("../data/day10.txt")
//         .lines()
//         .map(|line| {
//             let mut splitted = line.split(' ');
//             match splitted.next().unwrap() {
//                 "noop" => Instruction::Noop,
//                 "addx" => Instruction::Addx(splitted.next().unwrap().parse().unwrap()),
//                 _ => panic!("Unknown instruction"),
//             }
//         })
//         .for_each(|instruction| run(&instruction, &mut register));
//     let part1 = (20..221)
//         .step_by(SIZE)
//         .map(|i| (i as isize) * register[i - 1])
//         .sum::<isize>();

//     let mut screen: Vec<&str> = Vec::new();
//     let mut sprite_position = 1; // middle of the sprite
//     for (i, &x) in register[1..].iter().enumerate() {
//         let pixel_position = (i % SIZE) as isize;
//         if pixel_position.abs_diff(sprite_position) < 2 {
//             screen.push("\u{2593}");
//         } else {
//             screen.push("\u{0020}");
//         }
//         sprite_position = x;
//     }
//     println!("Part 1: {}", part1);
//     println!("Part 2:");
//     (0..screen.len())
//         .step_by(SIZE)
//         .for_each(|i| println!("{}", screen[i..i + SIZE].join("")));
//     println!(" ");
//     Ok(())
// }