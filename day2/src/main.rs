use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main(){
    let mut i1: usize = 0;
    let mut i2: usize = 0;
    let mut i3: usize = 0;
    let mut i4: usize = 0;
    let mut i5: usize = 0;
    let mut count1: u8 = 0;
    let mut count2: u8 = 0;
    let mut count3: u8 = 0;
    let mut count4: u8 = 0;
    let mut count5: u8 = 0;
    let lat1: usize = 1;
    let lat2: usize = 3;
    let lat3: usize = 5;
    let lat4: usize = 7;
    let lat5: usize = 1;
    let mut even: bool = true;
    if let Ok(lines) = read_lines("./data.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let chars = ip.chars().collect::<Vec<_>>();
                let len = ip.len();
                if chars[i1] == '#' {
                    count1 += 1;
                }
                if chars[i2] == '#' {
                    count2 += 1;
                }
                if chars[i3] == '#' {
                    count3 += 1;
                }
                if chars[i4] == '#' {
                    count4 += 1;
                }
                if even {
                    if chars[i5] == '#' {
                        count5 += 1;
                    }
                    i5 += lat5;
                    i5 = i5 % len;
                }
                i1 += lat1;
                i1 = i1 % len;
                i2 += lat2;
                i2 = i2 % len;
                i3 += lat3;
                i3 = i3 % len;
                i4 += lat4;
                i4 = i4 % len;
                even = !even;
            }
        }
    }
    println!("Slope 1 : {}\nSlope 2 : {}\nSlope 3 : {}\nSlope 4 : {}\nSlope 5 : {}\n", count1, count2, count3, count4, count5);
    println!("{}", (count1 as u32)*(count2 as u32)*(count3 as u32)*(count4 as u32)*(count5 as u32));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
