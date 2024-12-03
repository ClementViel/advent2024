use std::fs;

fn readline(path: &str) -> Vec<String> {

    fs::read_to_string(path) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
       .map(String::from)  // make each slice into a string
       .collect()  // gather them together into a vector
}

fn line_is_gradient(line: &Vec<i32>) -> bool {
    let mut ret = true;
    let sign_plus: bool = line[0] < line[1];
    if sign_plus {
        for idx in 0..line.len()-1 {
            ret &= line[idx] < line[idx+1];
        }
    } else {
        for idx in 0..line.len()-1 {
            ret &= line[idx] > line[idx+1];
        }

    }
    ret
}

fn gradient_is_bounded(line: &Vec<i32>) -> bool {
    let mut ret = true;
    let boundary_min = 1;
    let boundary_max = 3;
    for idx in 0..line.len()-1 {
        let sub = (line[idx] - line[idx+1]).abs();
        ret &= sub >= boundary_min;
        ret &= sub <= boundary_max;
    }
    ret
}

fn line_is_safe(line: Vec<i32>) -> bool {
    let mut ret = true;
    if line_is_gradient(&line) == true {
        ret &= gradient_is_bounded(&line);
    } else {
        ret = false
    }
    ret 

}

pub fn day2() {
    let filevec = readline("/Users/clem/Projets/prog/advent2024/advent2024/input2.txt");
    let mut safe = 0;
    for line in filevec.iter() {
        let line_int: Vec<i32> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        if line_is_safe(line_int) == true {
            safe +=1;
        }
    }
    println!("{}", safe);
}
