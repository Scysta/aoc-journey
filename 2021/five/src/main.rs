use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").expect("Path should exist.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Input should be valid UTF-8.");
    let mut lines = contents.lines();

    let mut next = lines.next();
    let mut counters = Vec::new();
    let mut number_of_lines = 0;

    while next != None {
        let num = &next
            .expect("String should not be empty.")[..];
        counters.resize(num.len(), 0);
        for i in 0..num.len() {
            counters[i] += &num[i..i+1].parse().expect("Bit should be a valid number."); 
        }
        number_of_lines += 1;
        next = lines.next();
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for i in counters {
        if number_of_lines/i > 1 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let power_con: isize = isize::from_str_radix(&gamma[..], 2).expect("Bad radix.")
        * isize::from_str_radix(&epsilon[..], 2).expect("Bad radix.");

    println!("The power consumption is: {}", power_con)
}
