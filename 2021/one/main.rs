use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut counter = 0;
    let mut file = File::open("test.txt").expect("Path doesn't exists.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file.");
    let mut lines = contents.lines();
    let mut prev = lines.next().expect("File is empty.");
    let mut next = lines.next();
    while next != None {
        let one: u16 = prev.parse().expect("Error");
        let two: u16 = next.expect("Error").parse().expect("Error");
        if two > one {
            counter += 1;
        };
        prev = next.expect("Error");
        next = lines.next();
    }
    println!("{}", counter);
}
