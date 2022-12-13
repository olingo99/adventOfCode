use crate::input;
use std::collections::HashMap;
use regex::RegexSet;
struct Monkey{
    id: u16,
    items: Vec<u32>,
    operation: String,
}

impl Monkey {
    fn new(id: u16, items: Vec<u32>, operation: String) -> Monkey{
        Monkey {
            id,
            items,
            operation,
        }
    }
    fn operation(&mut self){
        let data = self.operation.split_whitespace().collect::<Vec<&str>>();

        let set = RegexSet::new(&[
            r"old \* \d+",
            r"old \* old",
            r"old \+ \d+",
        ]).unwrap();
        dbg!(&self.operation);
        //let res = set.is_match(&self.operation);
        let res: Vec<_> = set.matches(&self.operation).into_iter().collect();
        dbg!(&res);
        match res[0]{
            0 => {
                let num = data[2].parse::<u32>().unwrap();
                self.items[0] = self.items[0] * num;
                dbg!(&self.items);
            },
            1 => {
                self.items[0] = self.items[0].pow(2);
                dbg!(&self.items);
            },
            2 => {
                let num = data[2].parse::<u32>().unwrap();
                self.items[0] = self.items[0] + num;
                dbg!(&self.items);
            },
            _ => {
                dbg!("No match");
            }
        }
    }
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
            temp.push(elem.trim().parse::<u32>().unwrap());
        }
        let mut monk = Monkey{
            id: monkeys.len() as u16,
            items:temp.clone(),
            operation: lines[2].split("=").collect::<Vec<&str>>().last().unwrap().trim().to_string(),
        };
        monkeys.push(monk);
        let x = monkeys.len();
        // let mut test = monkeys[x-1];
        let test = monkeys.last_mut().unwrap();
        test.operation();
        let monkey = &mut monkeys[x-1];
        //monkey.items[0] += 19

        monkey.operation();
        dbg!(&monkeys.last().unwrap().items);
    }

    Ok(())
}
