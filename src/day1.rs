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

// day 1 exercise
fn compute(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    //assume vec1 and vec2 are equal size (for the sake of the exercise)
    let mut sum = 0;
    for idx in 0..vec1.len() {
        sum += (vec1[idx] - vec2[idx]).abs();
    }
    sum
}


//day2 exercise.
fn similarity(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    let mut num_prec = 0;
    let mut sum_prec = 0;
    let mut sum = 0;
    let mut mult =0;
    for num in vec2.iter() {
        //cache mecanism in case of 2 similar numbers in left list
        if *num == num_prec {
            sum += sum_prec;
        } else {
            for num2 in vec1.iter() {
                // breaking if num from right is greater than num from left
                // to avoid going through all values.
                if *num2 > *num {
                    break;
                }
                if *num2 == *num {
                    mult += 1;
                } 
            }
        }
        sum += *num * mult;
        num_prec = *num;
        sum_prec = sum;
        mult = 0;
    }
    sum
}

fn main() {
    let fileVec: Vec<String> = readline("/Users/clem/Projets/prog/advent2024/advent2024/input1.txt");
    let (vec1, vec2) = splitVec(fileVec);
    // uncomment days to avoid dealing with borrowing issues
    // DAY 1 
 //   let sum = compute(vec1, vec2);
    // DAY 2
    let simi_sum = similarity(vec1, vec2);
    println!("{}", simi_sum);
}
