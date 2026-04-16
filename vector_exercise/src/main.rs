//Given a list of integer use a vectgor and return the mean media and mode (a hash map will be usefull here) of the list

use std::collections::HashMap;

fn main() {
    let arr: Vec<i32> = vec![1, 2, 3, 4, 5, 5, 5, 6, 7];

    let len: usize = arr.len();

    let median: i32 = if (len % 2) == 0 {
        let temp_index: usize = len / 2;
        (arr[temp_index] + arr[temp_index - 1]) / 2
    } else {
        arr[len % 2] 
    };

    println!("Median {}", median);

    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut max: usize = 0;
    let mut max_key: i32 = 0;

    for i in &arr {
        let count: &mut usize = map.entry(*i).or_insert(0);
        *count += 1;

        if *count > max {
            max_key = *i;
            max = *count
        }

    }

    let freq: usize = match map.get(&max_key) {
        None => 0,
        Some(s) => *s
    };

    println!("The mode is {} and its frequency {}", max_key, freq);

}
