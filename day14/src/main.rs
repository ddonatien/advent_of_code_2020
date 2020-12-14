extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

// Part1
// fn main() {
//     let mask_re = Regex::new(r"mask = ([X10]{36})").unwrap();
//     let mem_re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
//     let mut mem: HashMap<String, u64> = HashMap::new();
//     let mut mask: String = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
//     if let Ok(lines) = read_lines("./data.txt") {
//         for line in lines {
//             if let Ok(ip) = line {
//                 if let Some(cap) = mem_re.captures(&ip) {
//                     let val = cap[2].parse::<u64>().unwrap();
//                     let val_bin: Vec<char> = format!("{:036b}", val).chars().collect();
//                     let mut tmp = String::from("");
//                     for (i, c) in mask.chars().enumerate() {
//                         match c {
//                             'X' => tmp.push(val_bin[i]),
//                             _   => tmp.push(c)
//                         }
//                     }
//                     mem.insert(String::from(&cap[1]), u64::from_str_radix(&tmp, 2).unwrap());
//                 } else if let Some(cap) = mask_re.captures(&ip) {
//                     mask = String::from(&cap[1]);
//                 }
//             }
//         }
//     }
//     println!("Mem : {}", mem.values().sum::<u64>())
// }

fn main() {
    let mask_re = Regex::new(r"mask = ([X10]{36})").unwrap();
    let mem_re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let mut mem: HashMap<String, u64> = HashMap::new();
    let mut mask: String = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    if let Ok(lines) = read_lines("./data.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = mem_re.captures(&ip) {
                    let val = cap[2].parse::<u64>().unwrap();
                    let addr = cap[1].parse::<u64>().unwrap();
                    let addr_bin: Vec<char> = format!("{:036b}", addr).chars().collect();
                    let mut tmp = vec![String::from(""),];
                    for (i, c) in mask.chars().enumerate() {
                        match c {
                            '0' => {
                                for s in tmp.iter_mut() {
                                    s.push(addr_bin[i]);
                                }
                            },
                            '1' => {
                                for s in tmp.iter_mut() {
                                    s.push('1');
                                }
                            },
                            'X' => {
                                let mut ttmp = vec![];
                                for s in tmp.iter_mut() {
                                    let mut sc = s.clone();
                                    sc.push('0');
                                    s.push('1');
                                    ttmp.push(sc);
                                }
                                tmp.extend(ttmp);
                            },
                            _  => {},
                        }
                    }
                    for s in tmp {
                        mem.insert(s.clone(), val);
                    }
                } else if let Some(cap) = mask_re.captures(&ip) {
                    mask = String::from(&cap[1]);
                }
            }
        }
    }
    println!("Mem : {}", mem.values().sum::<u64>());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
