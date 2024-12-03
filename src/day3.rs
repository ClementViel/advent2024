#![feature(str_split_remainder)]
use std::fs;
use regex::Regex;

fn readline(path: &str) -> Vec<String> {

    fs::read_to_string(path) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
       .map(String::from)  // make each slice into a string
       .collect()  // gather them together into a vector
}

fn regex_line(line: &str) -> i32 {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let re2 = Regex::new(r"[0-9]+,[0-9]+").unwrap();
    let mut sum_line = 0;
    let multiplications: Vec<_> = re.find_iter(line).map(|x| x.as_str()).collect();
    for mult in multiplications.iter() {
        let members: Vec<_> = re2.find_iter(mult).map(|x| x.as_str()).collect();
        for member in members.iter() {
            let mut split = member.split(",");
            let elem_left: i32 = split.next().unwrap().to_string().parse().expect("aie");
            let elem_right: i32 = split.next().unwrap().to_string().parse().expect("aie");
            sum_line += elem_left * elem_right;
        }
    }
    sum_line
}

pub fn day3() {
    let mut result = 0;
    let lines = readline("/Users/clem/Projets/prog/advent2024/advent2024/input3.txt");
    for line in lines.iter() {
        result += regex_line(line);
    }
    println!("{:?}", result);
}
