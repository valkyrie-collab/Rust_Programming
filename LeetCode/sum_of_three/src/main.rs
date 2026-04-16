fn merge(arr: &mut [i32], low: usize, high: usize, mid: usize) {
    let len_one: usize = mid - low + 1;
    let len_two: usize = high - mid;

    let mut arr_one: Vec<i32> = vec![0; len_one];
    let mut arr_two: Vec<i32> = vec![0; len_two];

    for i in 0..len_one {
        arr_one[i] = arr[low + i];
    }

    for j in 0..len_two {
        arr_two[j] = arr[mid + j + 1];
    }

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut k: usize = low;

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
    let mut arr_of_numbers: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];
    let len: usize = arr_of_numbers.len();

    sort(&mut arr_of_numbers, 0, len - 1);

    println!("The value of array: {:?}", arr_of_numbers);

    
}
