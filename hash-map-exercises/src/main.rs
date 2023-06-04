use std::collections::HashMap;

fn main() {
    let mut data = vec![1,2,3,4,5,6,7];
    
    println!("Mean: {}", mean(&data));
}

fn mean(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in numbers {
        sum += num
    }
    sum / numbers.len() as i32

}
