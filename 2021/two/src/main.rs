use std::io::prelude::*;
use std::fs::File;

fn insert(window: &mut [u32; 3], x: u32) {
    window[0] = window[1];
    window[1] = window[2];
    window[2] = x;
}

fn sum(window: &[u32; 3]) -> u32 {
    window[0] + window[1] + window[2]
}

fn main() {
    let mut counter = 0;
    let mut file = File::open("input.txt").expect("Path should exist.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Input should be valid UTF-8.");
    let mut lines = contents.lines();
    
    let mut window1: [u32; 3] = [0, 0, 0];

    for i in 0..3 {
        let x = lines
            .next()
            .expect("Input should not be empty.")
            .parse()
            .expect("Input should be a number.");
        window1[i] = x;
    }

    let mut next = lines.next();
    let mut window2: [u32; 3] = window1;

    while next != None {
        let num = next
            .expect("Input should not be empty.")
            .parse()
            .expect("Input should be a number.");
        window1 = window2;
        insert(&mut window2, num);
        if sum(&window1) < sum(&window2) {
            counter += 1;
        }
        next = lines.next();
    };

    println!("{}", counter);
}
