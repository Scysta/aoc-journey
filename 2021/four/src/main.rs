use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").expect("Path should exist.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Input should be valid UTF-8.");
    let mut lines = contents.lines();

    let mut next = lines.next();
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    
    while next != None {
        let order = next
            .expect("Iterator should not be None.")
            .split_once(" ")
            .expect("Delimiter should be present in string.");
        let num: u32 = order.1
            .parse()
            .expect("String contents is not a valid number.");
        match order.0 {
            "up" => aim -= num,
            "down" => aim += num,
            _ => {
                pos += num;
                depth += aim * num;
            }
        }
        next = lines.next();
    };

    println!("Depth: {}, Position: {}", depth, pos);
    println!("{}", depth * pos);
}
