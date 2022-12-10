use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();
    let mut score = 0;
    for round in lines {
        let mut play = round.split_whitespace();
        // Not my brightest moment...
        match play.next() {
            Some("A") => match play.next() {
                Some("X") => score += 3 + 1,
                Some("Y") => score += 6 + 2,
                Some("Z") => score += 0 + 3,
                _ => todo!()
            },
            Some("B") => match play.next() {
                Some("Y") => score += 3 + 2,
                Some("Z") => score += 6 + 3,
                Some("X") => score += 0 + 1,
                _ => todo!()
            },
            Some("C") => match play.next() {
                Some("Z") => score += 3 + 3,
                Some("X") => score += 6 + 1,
                Some("Y") => score += 0 + 2,
                _ => todo!()
            },
            _ => todo!()
        }
    }
    println!("{}", score);
}
