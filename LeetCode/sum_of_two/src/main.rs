use std::collections::HashMap;

fn solution(arr: &[i32], target: i32) -> Vec<usize> {
    let mut one_index: usize = 1;
    let mut hash_map: HashMap<i32, usize> = HashMap::new();

    for i in arr.iter() {
        let val: i32 = target - *i; //9 - 7 = 2

        if hash_map.contains_key(&val) {
            let first: usize = *hash_map.get(&val).unwrap();
            let second: usize = one_index;

            return vec![first, second];
        }

        hash_map.insert(*i, one_index);
        one_index += 1;
    }

    vec![0, 0]
}

fn main() {
    let arr: Vec<i32> = vec![-1, 0];
    let sol: Vec<usize> = solution(&arr, -1);

    println!("The value of solution is: {:?}", sol);
}
