#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};

fn main() {
    let fields1: HashSet<String> = [
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid"),
        String::from("cid")].iter().cloned().collect();
    let fields2: HashSet<String> = [
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid")].iter().cloned().collect();
    let mut count1: u16 = 0;
    let mut count2: u16 = 0;
    if let Ok(lines) = read_lines("./data.txt") {
        let mut psprt: HashMap<String, String> = HashMap::new();
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() > 0 {
                    let tmp = ip.split(' ')
                                .map(|i| {let a = i.split(':').collect::<Vec<&str>>(); (a[0], a[1])})
                                .collect::<HashMap<&str, &str>>();
                    for (key, value) in tmp{
                        psprt.insert(String::from(key), String::from(value));
                    }
                } else {
                    let set1: HashSet<String> = psprt.keys().cloned().collect();
                    if (fields1.intersection(&set1).cloned().collect::<HashSet<String>>() == fields1) |
                       (fields2.intersection(&set1).cloned().collect::<HashSet<String>>() == fields2) {
                        count1 += 1;
                        if check_fields(psprt) {
                            count2 += 1;
                        }
                    }
                    psprt = HashMap::new();
                }
            }
        }
        let set1: HashSet<String> = psprt.keys().cloned().collect();
        if (fields1.intersection(&set1).cloned().collect::<HashSet<String>>() == fields1) |
            (fields2.intersection(&set1).cloned().collect::<HashSet<String>>() == fields2) {
            count1 += 1;
            if check_fields(psprt) {
                count2 += 1;
            }
        }
    }
    println!("{} valid passports", count1);
    println!("{} double valid passports", count2);
}

fn check_fields(psprt: HashMap<String, String>) -> bool {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"(\d+)(cm|in)").unwrap();
        static ref RE2: Regex = Regex::new(r"#[0-9|a-f]{6}$").unwrap();
        static ref RE3: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    let byr: u16 = psprt["byr"].parse().unwrap();
    if (byr < 1920) | (byr > 2002) {
        return false
    }
    let iyr: u16 = psprt["iyr"].parse().unwrap();
    if (iyr < 2010) | (iyr > 2020) {
        return false
    }
    let eyr: u16 = psprt["eyr"].parse().unwrap();
    if (eyr < 2020) | (eyr > 2030) {
        return false
    }
    match  RE1.captures(&psprt["hgt"]){
        Some(cap) => {
            let hgt: u16 = cap[1].parse().unwrap();
            if &cap[2] == "cm" {
                if (hgt < 150) | (hgt > 193) {
                    return false;
                }
            } else if &cap[2] == "in" {
                if (hgt < 59) | (hgt > 76) {
                    return false;
                }
            } else {
                return false;
            }
        },
        None      => return false,
    }
    let hcl: &str = &psprt["hcl"];
    if !RE2.is_match(hcl) {
        return false;
    }
    let ecl: &str = &psprt["ecl"];
    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|&i| i==ecl) {
        return false;
    }
    let pid: &str = &psprt["pid"];
    if !RE3.is_match(pid) {
        return false;
    }
    return true;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
