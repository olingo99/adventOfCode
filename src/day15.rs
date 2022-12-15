use crate::input;
use std::collections::{HashMap, HashSet};

pub fn day15() -> input::Result<()> {
    let target_line = 10;
    let mut set_a : HashMap<(i32,i32),i32>  = HashMap::new();
    let mut set_b : HashMap<(i32,i32),i32>  = HashMap::new();
    let contents = input::load_day_file("day15.txt");
    let mut map :HashMap<(i32,i32), char> = HashMap::new();
    let mut max_manhatan : HashMap<(i32,i32), i32> = HashMap::new();
    for line in contents.lines(){
        let data = line.split(":").collect::<Vec<&str>>();
        let sensor_data = data[0].split("=").collect::<Vec<&str>>();
        let beacon_data = data[1].split("=").collect::<Vec<&str>>();
        let sensor = (sensor_data[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(), sensor_data[2].parse::<i32>().unwrap());
        // dbg!(&beacon_data);
        let beacon = (beacon_data[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(), beacon_data[2].parse::<i32>().unwrap());
        // dbg!(sensor);
        // dbg!(beacon);
        map.insert(sensor, 'S');
        map.insert(beacon, 'B');

        max_manhatan.insert(sensor, manhattan_distance(sensor, beacon));

        set_a.insert(sensor, sensor.1-sensor.0+manhattan_distance(sensor, beacon)+1);
        set_a.insert(sensor, sensor.1-sensor.0-manhattan_distance(sensor, beacon)-1);
        set_b.insert(sensor, sensor.1+sensor.0+manhattan_distance(sensor, beacon)+1);
        set_b.insert(sensor, sensor.1+sensor.0-manhattan_distance(sensor, beacon)-1);
    }
    let mut points:HashSet<(i32,i32)> = HashSet::new();
    for elem in set_a.values(){
        for elem2 in set_b.values(){
            let point = ((elem2-elem)/2,(elem+elem2)/2);
            if point.0>0 && point.1>0 && point.0<4000000 && point.1<4000000{
                points.insert(point);
            }
        }
    }

    for point in points.clone(){
            for sensor in max_manhatan.clone(){
                if manhattan_distance(point, sensor.0)<=sensor.1{
                    //dbg!("eeeeeeeeeeeeeeeeeee");
                    points.remove(&point);
                }
            }
        }
    
    // let mut test = HashSet::new();
    // for point in points.clone(){
    //     for sensor in max_manhatan.clone(){
    //         if manhattan_distance(point, sensor.0)<=sensor.1{
    //             //dbg!("eeeeeeeeeeeeeeeeeee");
    //             test.insert(point);
    //         }
    //     }
    // }
    // dbg!(&test);
    // dbg!(&points);
    // for elem in points{
    //     if !test.contains(&elem){
    //         dbg!(elem);
    //     }
    // }
    for elem in points{
        let res:u64 = elem.0 as u64 * 4000000 as u64 + elem.1 as u64;
        dbg!(res);
    }

    //dbg!(points.len());
    Ok(())
}

fn manhattan_distance(a: (i32,i32), b: (i32,i32)) -> i32{
    (a.0-b.0).abs()+(a.1-b.1).abs()
}


pub fn part1() -> input::Result<()>{
    let target_line = 10;

    let contents = input::load_day_file("day15.txt");
    let mut map :HashMap<(i32,i32), char> = HashMap::new();
    let mut max_manhatan : HashMap<(i32,i32), i32> = HashMap::new();
    for line in contents.lines(){
        let data = line.split(":").collect::<Vec<&str>>();
        let sensor_data = data[0].split("=").collect::<Vec<&str>>();
        let beacon_data = data[1].split("=").collect::<Vec<&str>>();
        let sensor = (sensor_data[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(), sensor_data[2].parse::<i32>().unwrap());
        // dbg!(&beacon_data);
        let beacon = (beacon_data[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(), beacon_data[2].parse::<i32>().unwrap());
        // dbg!(sensor);
        // dbg!(beacon);
        map.insert(sensor, 'S');
        map.insert(beacon, 'B');

        max_manhatan.insert(sensor, manhattan_distance(sensor, beacon));
    }
    let mut set : HashSet<(i32,i32)>  = HashSet::new();
    let mut to_check : HashSet<(i32,i32)>  = HashSet::new();
    let mut used : HashSet<(i32,i32)>  = HashSet::new();
    for elem in max_manhatan.keys(){
        if manhattan_distance(*elem, (elem.0,target_line))<=max_manhatan[&elem]{
                    //dbg!(&elem);
        let dist = (elem.1-target_line).abs();
        // dbg!(&dist);
        // dbg!(max_manhatan[&elem]);
        if !used.contains(&(elem.0, target_line)){
            used.insert((elem.0, target_line));
        }
        for i in 0..max_manhatan[elem]-dist+1{
            if !used.contains(&(elem.0-i, target_line)){
                used.insert((elem.0-i, target_line));
            }
            
            if !used.contains(&(elem.0+i, target_line)){
                used.insert((elem.0+i, target_line));
            }
        }
        // dbg!(used.len());
        // dbg!(&used);
        }
    }

    //dbg!(&used);
    for elem in to_check{

        
    }
    for elem in max_manhatan.keys(){
        if elem.1 == target_line{
            used.insert(*elem);
        }
    }
    //dbg!(&max_manhatan);
    dbg!(used.len());
    Ok(())
}
//manhattan distance
