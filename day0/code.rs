use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main(){
    let mut comps: Vec<u32> = Vec::new();
    let mut tmps: Vec<u32> = Vec::new();
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let nb: u32 = ip.parse().unwrap();
                if comps.iter().any(|&i| i==nb) {
                    println!("Found couple → {} {}", nb, 2020 - nb);
                    println!("Result {}", nb*(2020 - nb));
                    break;
                }
                else {
                    comps.push(2020-nb);
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
