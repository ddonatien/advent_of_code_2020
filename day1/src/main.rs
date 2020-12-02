extern crate regex;

use std::fs::File;
use regex::Regex;
use std::io::{self, BufRead};
use std::path::Path;

fn main(){
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let mut count: u16 = 0;
    if let Ok(lines) = read_lines("./data.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let cap = re.captures(&ip).unwrap();
                let alpha: u8 = cap[1].parse::<u8>().unwrap();
                let beta: u8 = cap[2].parse::<u8>().unwrap();
                let l: char = cap[3].chars().next().unwrap();
                // if check_pwd_1(alpha, beta, l, &cap[4]) {
                //     count +=1;
                // }
                if check_pwd_2(alpha as usize, beta as usize, l, &cap[4]) {
                    count +=1;
                }
            }
        }
    }
    println!("{} valid passwords", count);
}

fn check_pwd_1(min: u8, max: u8, l: char, pwd: &str) -> bool {
    let mut count: u8 = 0;
    let res: bool;
    for c in pwd.chars() {
        if c == l {
            count +=1;
        }
    }
    if (count >= min) & (count <= max) {
        res = true;
    }
    else {
        res = false;
    }
    return res;
}

fn check_pwd_2(pos1: usize, pos2: usize, l: char, pwd: &str) -> bool {
    let mut res: bool = false;
    let chars: Vec<char> = pwd.chars().collect();
    if chars[pos1 - 1] == l {
        res = !res;
    }
    if chars[pos2 - 1] == l {
        res = !res;
    }
    return res;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
