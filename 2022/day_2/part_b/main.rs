use std::io::Read;
use std::fs::File;

fn main() {
    let mut file = File::open("../part_a/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();
    let mut score = 0;
    for line in lines {
        let mut play = line.split_whitespace();
        match play.next().unwrap() {
            "A" => match play.next().unwrap() {
                "X" => score += 0 + 3,
                "Y" => score += 3 + 1,
                "Z" => score += 6 + 2,
                _ => todo!()
            },
            "B" => match play.next().unwrap() {
                "X" => score += 0 + 1,
                "Y" => score += 3 + 2,
                "Z" => score += 6 + 3,
                _ => todo!()
            },
            "C" => match play.next().unwrap() {
                "X" => score += 0 + 2,
                "Y" => score += 3 + 3,
                "Z" => score += 6 + 1,
                _ => todo!()
            },
            _ => todo!()
        }
    }
    println!("{}", score);
}
