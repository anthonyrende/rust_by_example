use std::collections::HashMap;

/*
Given a list of integers, use a vector and return the median 
(when sorted, the value in the middle position) and mode 
(the value that occurs most often; a hash map will be helpful here) 
of the list.
 */
fn main() {
    let mut data = vec![1,2,3,4,5,6,7,7];
    
    println!("Mean: {}", mean(&data));
    // Note: a mutable borrow is necessary here because sorting the numbers is a mutating operation
    println!("Median: {}", median(&mut data));
    println!("Mode: {:?}", mode(&data).unwrap_or(0));
}

fn mean(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in numbers {
        sum += num
    }
    sum / numbers.len() as i32

}

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

fn mode(numbers: &Vec<i32>) -> Option<i32> {
    let mut mode_map = HashMap::new();

    for num in numbers {
        let count = mode_map.entry(num).or_insert(0);
        *count += 1
    }

    let mut mode = None;
    let mut highest_freq_count = 0;

    for (num, &freq) in mode_map.iter(){
        if freq > highest_freq_count {
            highest_freq_count = freq;
            mode = Some(*num);
        }
    }
    mode.copied()
}