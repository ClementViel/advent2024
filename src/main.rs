use std::fs;

fn readline(path: &str) -> Vec<String> {

    fs::read_to_string(path) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
       .map(String::from)  // make each slice into a string
       .collect()  // gather them together into a vector
}

fn binaryInsert(elem1: i32, vectorDest: Vec<i32>) {
}

fn splitVec(vector: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut vecLeft: Vec<i32> = vec![];
    let mut vecRight: Vec<i32> = vec![];
    for line in vector.iter() {
        let num: Vec<String> = line.split("   ").map(|num| num.to_string()).collect();
        let elemRight: i32 = num[0].parse().unwrap();
        let elemLeft: i32 = num[1].parse().unwrap();
        let indexRight = vecRight.partition_point(|&x| x < elemRight);
        let indexLeft = vecLeft.partition_point(|&x| x < elemLeft);
            vecRight.insert(indexRight, elemRight);
            vecLeft.insert(indexLeft, elemLeft);
        }
    (vecLeft, vecRight)
}

fn compute(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    //assume vec1 and vec2 are equal size (for the sake of the exercise)
    let mut sum = 0;
    for idx in 0..vec1.len() {
        sum += (vec1[idx] - vec2[idx]).abs();
    }
    sum
}

fn main() {
    let fileVec: Vec<String> = readline("/Users/clem/Projets/prog/advent2024/advent2024/input1.txt");
    let (vec1, vec2) = splitVec(fileVec);
    let sum = compute(vec1, vec2);
    println!("{}", sum);
}
