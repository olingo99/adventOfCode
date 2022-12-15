use crate::input;
use std::collections::{HashMap, HashSet};

pub fn day15() -> input::Result<()> {
    let target_line = 10;

    let contents = input::load_day_file("day15.txt");
    let mut map :HashMap<(i32,i32), char> = HashMap::new();
    let mut max_manhatan : HashMap<(i32,i32), i32> = HashMap::new();
    for line in contents.lines(){
        let data = line.split(":").collect::<Vec<&str>>();
        let sensor_data = data[0].split("=").collect::<Vec<&str>>();
        let beacon_data = data[1].split("=").collect::<Vec<&str>>();
        let sensor = (sensor_data[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(), sensor_data[2].parse::<i32>().unwrap());
        dbg!(&beacon_data);
        let beacon = (beacon_data[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(), beacon_data[2].parse::<i32>().unwrap());
        dbg!(sensor);
        dbg!(beacon);
        map.insert(sensor, 'S');
        map.insert(beacon, 'B');

        max_manhatan.insert(sensor, manhattan_distance(sensor, beacon));
    }
    let set : HashSet<(i32,i32)>  = HashSet::new();
    for 
    dbg!(&max_manhatan);
    Ok(())
}

fn manhattan_distance(a: (i32,i32), b: (i32,i32)) -> i32{
    (a.0-b.0).abs()+(a.1-b.1).abs()
}

//manhattan distance
