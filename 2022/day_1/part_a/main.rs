use std::io::Read;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();
    let mut curr = 0;
    let mut max = 0;

    for line in lines {
        match line {
            "" => {
                if curr > max {
                    max = curr;
                }
                curr = 0;
            },
            num => {
                curr += num.trim().parse::<u32>().unwrap();
            }   
        }
    }
    println!("{}", max);
}
