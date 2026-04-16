use std::collections::HashMap;

fn merge(arr: &mut [i32], low: usize, high: usize, mid: usize) {
    let len_one: usize = mid - low + 1;
    let len_two: usize = high - mid;

    let mut arr_one: Vec<i32> = vec![0; len_one];
    let mut arr_two: Vec<i32> = vec![0; len_two];

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut k: usize = low;

    while i < len_one {
        arr_one[i] = arr[low + i];
        i += 1;
    }

    while j < len_two {
        arr_two[j] = arr[mid + j + 1];
        j += 1;
    }

    i = 0;
    j = 0;

    while i < len_one && j < len_two {

        if arr_one[i] < arr_two[j] {
            arr[k] = arr_one[i];
            i += 1;
        } else {
            arr[k] = arr_two[j];
            j += 1;
        }

        k += 1;
    }

    while i < len_one {
        arr[k] = arr_one[i];
        i += 1;
        k += 1;
    }

    while j < len_two {
        arr[k] = arr_two[j];
        j += 1;
        k += 1;
    }
}

fn sort(arr: &mut [i32], low: usize, high: usize) {

    if low >= high {
        return;
    }

    let mid: usize = low + (high - low) / 2;

    sort(arr, low, mid);
    sort(arr, mid + 1, high);
    merge(arr, low, high, mid);
}

fn main() {
    let mut arr: Vec<i32> = vec![40, 10, 30, 20];
    let len: usize = arr.len();
    let mut hash_map: HashMap<i32, usize> = HashMap::with_capacity(len);
    let mut k: usize = 1;

    for i in &arr {
        hash_map.insert(*i, k);
        k += 1;
    }

    sort(&mut arr, 0, 3);
    println!("{:?}", arr);

    let mut rank_arr: Vec<usize> = vec![0; len];

    for i in 0..len {
        rank_arr[i] = *(hash_map.get(&arr[i]).unwrap());
    }

    println!("{:?}", rank_arr);
}
