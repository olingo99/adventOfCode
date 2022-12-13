use crate::input;
use std::collections::HashMap;

struct Monkey {
    id: u16,
    items: Vec<u32>,
    operation: String,
}

pub fn day11() -> input::Result<()> {
    let contents = input::load_day_file("day11.txt");
    let mut monkeys = Vec::new();
    // for line in contents.lines() {
    //     if line.is_empty() {
    //         continue;
    //     }
    //     let mut parts = line.split_whitespace().collect::<Vec<&str>>();
    //     match parts[0]{
    //         "Monkey" => {
    //             let id = parts[1].parse::<u16>().unwrap();
    //             let items = parts[2].split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    //             let operation = parts[3].to_string();
    //             let monkey = Monkey{id, items, operation};
    //             monkeys.push(monkey);
    //         },
    //     }
    //     //if parts[0] == "Monkey"{

    //     }
    // }
    let people = contents.split( "\r\n\r\n" ).collect::<Vec<&str>>();
    for block in people{
        let lines = block.split("\r\n").collect::<Vec<&str>>();
        let split = lines[1].split(":").collect::<Vec<&str>>().last().unwrap().split(",").collect::<Vec<&str>>();
        let mut temp = vec![];
        for elem in split{
            temp.push(elem.trim().parse::<u32>().unwrap());
        }
        monkeys.push(Monkey{
            id: monkeys.len() as u16,
            items: temp.clone(),
            operation: "ee".to_string(),
        });
        // dbg!(&monkeys.last().unwrap().id);
        dbg!(&monkeys.last().unwrap().items);
        // dbg!(&monkeys.last().unwrap().operation);
        // for line in lines{
        //     dbg!(line);
        // }
    }

    Ok(())
}
