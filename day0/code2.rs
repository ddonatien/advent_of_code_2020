use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main(){
    if let Ok(lines) = read_lines("./data.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let nb: u32 = ip.parse().unwrap();
                let res: u32 = find_couple(2020 - nb);
                if res > 0 {
                    println!("Result: {}", res*nb);
                    break;
                }
            }
        }
    }
}

fn find_couple(value: u32) -> u32 {
    let mut comps: Vec<u32> = Vec::new();
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if let Ok(nb) = ip.parse::<u32>() {
                    if value > nb {
                        if comps.iter().any(|&i| i==nb) {
                            return nb*(value - nb);
                        }
                        else {
                            comps.push(value - nb);
                        }
                    }
                }
            }
        }
    }
    return 0;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
