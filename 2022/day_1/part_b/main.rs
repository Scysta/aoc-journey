use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("../part_a/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();
    let mut curr = 0;

    let mut elfs = [0; 3];

    for line in lines {
        match line {
            "" => {
                for i in 0..2 {
                    if curr > elfs[i] {
                        elfs[i] = curr;
                        elfs.sort();
                        break;
                    }
                }
                curr = 0;
            },
            cal => {
                curr += cal.trim().parse::<u32>().unwrap();
            }
        }
    }
    println!("{}", elfs[0] + elfs[1] + elfs[2]);
}
