use crate::input;

pub fn day5() -> input::Result<()> {
    let contents = input::load_day_file("day5.txt");

    let nb_stacks = ((contents.lines().next().unwrap().len()) as f64 /(4 as f64)).ceil();
    dbg!(nb_stacks);

    let mut stack_start : bool = true;

    let mut stack_vec = Vec::new();
    for _i in 0..nb_stacks as usize {
        stack_vec.push(Vec::new());
    }

    for line in contents.lines(){
        if line.is_empty(){
            stack_start=false;
            for i in 0..nb_stacks as usize{
                stack_vec[i].reverse();
            }
            continue;
        }
        if stack_start{
            for i in 0..nb_stacks as usize{
                let char = line.chars().nth(1+i*4).unwrap();
                if char.is_alphabetic(){
                    stack_vec[i].push(char);
                }
            }
        }
        else{
            // let data = line.split(" ").collect::<Vec<&str>>();
            // let from = data[3].parse::<usize>().unwrap()-1;
            // let to = data[5].parse::<usize>().unwrap()-1;
            // let num = data[1].parse::<usize>().unwrap();
            // let stack = &stack_vec[data[3].parse::<usize>().unwrap()-1];
            // let to_push = &stack[stack.len()-1-data[1].parse::<usize>().unwrap().. stack.len()];
            // stack_vec[data[5].parse::<usize>().unwrap()-1].extend_from_slice(&to_push);
            // //dbg!(to_push);
            let data = line.split(" ").collect::<Vec<&str>>();
            let mut temp_vec = Vec::new();
            for _i in 0..data[1].parse::<usize>().unwrap(){
                temp_vec.push(stack_vec[data[3].parse::<usize>().unwrap()-1].pop().unwrap());
            }
            temp_vec.reverse();
            stack_vec[data[5].parse::<usize>().unwrap()-1].extend(temp_vec);

            // better to use slices i think, to do later
        }
    }

    for _i in 0..nb_stacks as usize{
        dbg!(stack_vec[_i][stack_vec[_i].len()-1]);
    }



    Ok(())
}

fn exo1() -> input::Result<()> {
    let contents = input::load_day_file("day5.txt");

    let nb_stacks = ((contents.lines().next().unwrap().len()) as f64 /(4 as f64)).ceil();
    dbg!(nb_stacks);

    let mut stack_start : bool = true;

    let mut stack_vec = Vec::new();
    for _i in 0..nb_stacks as usize {
        stack_vec.push(Vec::new());
    }

    for line in contents.lines(){
        if line.is_empty(){
            stack_start=false;
            for i in 0..nb_stacks as usize{
                stack_vec[i].reverse();
            }
            continue;
        }
        if stack_start{
            for i in 0..nb_stacks as usize{
                let char = line.chars().nth(1+i*4).unwrap();
                if char.is_alphabetic(){
                    stack_vec[i].push(char);
                }
            }
        }
        else{
            let data = line.split(" ").collect::<Vec<&str>>();
            for _i in 0..data[1].parse::<usize>().unwrap(){
                let to_push = stack_vec[data[3].parse::<usize>().unwrap()-1].pop().unwrap();
                stack_vec[data[5].parse::<usize>().unwrap()-1].push(to_push);
            }
        }
    }

    for _i in 0..nb_stacks as usize{
        dbg!(stack_vec[_i][stack_vec[_i].len()-1]);
    }



    Ok(())
}
