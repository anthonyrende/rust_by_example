use std::collections::HashMap;

fn main() {
    let mut data = vec![1,2,3,4,5,6,7];
    
    println!("Mean: {}", mean(&data));
    println!("Median: {}", median(&mut data));
}

fn mean(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in numbers {
        sum += num
    }
    sum / numbers.len() as i32

}
/*
Given a list of integers, use a vector and return the median 
(when sorted, the value in the middle position) and mode 
(the value that occurs most often; a hash map will be helpful here) 
of the list.
 */

fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();

    let middle = numbers.len() / 2;
    let median: i32 = 0;
    if numbers.len() % 2 == 0 {
        mean(&vec!(numbers[middle], numbers[middle - 1])) as i32
    }
    else {
        numbers[middle]
    }
    
    
}