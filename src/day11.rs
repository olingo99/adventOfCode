use crate::input;
use std::collections::HashMap;
use regex::RegexSet;
struct Monkey{
    counted : u32,
    items: Vec<u64>,
    operation: String,
    condition: u32,
    r_true: u16,
    r_false: u16,
}

impl Monkey {
    fn new( items: Vec<u64>, operation: String, condition: u32, r_true: u16, r_false : u16) -> Monkey{
        Monkey {
            counted : 0,
            items,
            operation,
            condition,
            r_true,
            r_false,
        }
    }
    fn operation(&mut self, supermodulo:& u64){
        let data = self.operation.split_whitespace().collect::<Vec<&str>>();

        let set = RegexSet::new(&[
            r"old \* \d+",
            r"old \* old",
            r"old \+ \d+",
        ]).unwrap();
        //dbg!(&self.operation);
        //let res = set.is_match(&self.operation);
        let res: Vec<_> = set.matches(&self.operation).into_iter().collect();
        for i in 0..self.items.len(){
            self.counted += 1;
            match res[0]{
                0 => {
                    let num = data[2].parse::<u32>().unwrap();
                    //self.items[i] = ((self.items[i] * num) as f64/3 as f64).floor() as u32;
                    self.items[i] = (self.items[i] * num as u64)%*supermodulo;
                    //dbg!(&self.items);
                },
                1 => {
                    //self.items[i] = ((self.items[i].pow(2)) as f64/3 as f64).floor() as u32;
                    self.items[i] = (self.items[i].pow(2))%*supermodulo;
                    //dbg!(&self.items);
                },
                2 => {
                    let num = data[2].parse::<u32>().unwrap();
                    // self.items[i] = ((self.items[i] + num) as f64/3 as f64).floor() as u32;
                    self.items[i] = (self.items[i] + num as u64)%*supermodulo;
                    //dbg!(&self.items);
                },
                _ => {
                    dbg!("No match");
                }
            }
        }
        // match res[0]{
        //     0 => {
        //         let num = data[2].parse::<u32>().unwrap();
        //         self.items[0] = ((self.items[0] * num) as f64/3 as f64).floor() as u32;
        //         //dbg!(&self.items);
        //     },
        //     1 => {
        //         self.items[0] = ((self.items[0].pow(2)) as f64/3 as f64).floor() as u32;
        //         //dbg!(&self.items);
        //     },
        //     2 => {
        //         let num = data[2].parse::<u32>().unwrap();
        //         self.items[0] = ((self.items[0] + num) as f64/3 as f64).floor() as u32;
        //         //dbg!(&self.items);
        //     },
        //     _ => {
        //         dbg!("No match");
        //     }
        // }
    }

}

fn condition_check(id:usize, monkeys: &mut Vec<Monkey>){
    //let me = &monkeys[id];
    let r_true = monkeys[id].r_true;
    let r_false = monkeys[id].r_false;
    for item in monkeys[id].items.clone(){
        if item%monkeys[id].condition as u64 == 0{
            monkeys[r_true as usize].items.push(item);
        }
        else{
            monkeys[r_false as usize].items.push(item);
        }
    }
    monkeys[id].items.clear();
}

pub fn day11() -> input::Result<()> {
    let contents = input::load_day_file("day11.txt");
    let mut monkeys = Vec::new();
    let people = contents.split( "\r\n\r\n" ).collect::<Vec<&str>>();
    for block in people{
        let lines = block.split("\r\n").collect::<Vec<&str>>();
        let split = lines[1].split(":").collect::<Vec<&str>>().last().unwrap().split(",").collect::<Vec<&str>>();
        let mut temp = vec![];
        for elem in split{
            temp.push(elem.trim().parse::<u64>().unwrap());
        }
        let mut monk = Monkey{
            counted: 0,
            items:temp.clone(),
            operation: lines[2].split("=").collect::<Vec<&str>>().last().unwrap().trim().to_string(),
            condition: lines[3].split_whitespace().collect::<Vec<&str>>().last().unwrap().trim().parse::<u32>().unwrap(),
            r_true: lines[4].split_whitespace().collect::<Vec<&str>>().last().unwrap().trim().parse::<u16>().unwrap(),
            r_false: lines[5].split_whitespace().collect::<Vec<&str>>().last().unwrap().trim().parse::<u16>().unwrap(),
        };
        monkeys.push(monk);
    }
    let mut supermodulo: u64 = 1;
    for j in 0..monkeys.len(){
        supermodulo *= monkeys[j].condition as u64;
    }

    for _i in 0..10000{
        for j in 0..monkeys.len(){
            if monkeys[j].items.len() > 0{
                monkeys[j].operation(&supermodulo);
                condition_check(j, &mut monkeys);
                //dbg!(&monkeys[j].items);
            }
        }
    }

    let mut res = Vec::new();
    for j in 0..monkeys.len(){
        //dbg!(&monkeys[j].items);
        res.push(monkeys[j].counted);
    }
    
    let max = &res.iter().max().unwrap();
    dbg!(&max);
    let index = res.iter().position(|x| *x == **max).unwrap();
    res.remove(index);
    dbg!(&res.iter().max());

    Ok(())
}
