use std::io::{self, Write};

fn merge(arr: &mut Vec<i32>, left: usize, right: usize, mid: usize) {
    let len_one: usize = mid - left + 1;
    let len_two: usize = right - mid;

    let mut arr_one: Vec<i32> = vec![0; len_one];
    let mut arr_two: Vec<i32> = vec![0; len_two];

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut k: usize = left;

    while i < len_one {
        arr_one[i] = arr[left + i];
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
        k += 1;
        i += 1;
    }

    while j < len_two {
        arr[k] = arr_two[j];
        k += 1;
        j += 1;
    }

}

fn sort(arr: &mut Vec<i32>, left: usize, right: usize) {

    if left >= right {
        return;
    }

    let mid: usize = left + (right - left) / 2;

    sort(arr, left, mid);
    sort(arr, mid + 1, right);
    merge(arr, left, right, mid);
}

fn print_sort(arr: &Vec<i32>, mut len: usize) {
    print!("[");

    for i in arr {

        if len <= 0 {
            break;
        }
        
        len -= 1;

        print!("{}, ", i);
    }

    println!("\x08\x08]");
}

fn get_arr(str_arr: String) -> (Vec<i32>, usize) {
    let len: usize = 100;
    let mut arr: Vec<i32> = vec![0; len];
    let mut true_len: usize = 1;
    let mut num: i32 = 0;

    for i in str_arr.chars() {

        if i >= '0' && i <= '9' {
            num = num * 10 + ((i as i32) - ('0' as i32));
        } else if i == ' ' {
            arr[true_len - 1] = num;
            true_len += 1;
            num = 0;
        }

        arr[true_len - 1] = num;
    }

    (arr, true_len)
}

fn main() {
    // let mut arr: Vec<i32> = vec![8, 7, 6, 5, 4, 3, 2, 1];

    print!("Enter the arr: ");
    io::stdout().flush().expect("Cannot clean read line");
    
    let mut str_arr: String = String::new();
    io::stdin().read_line(&mut str_arr).expect("Cannot read line");

    let arr_and_size: (Vec<i32>, usize) = get_arr(str_arr);

    let len: usize = arr_and_size.1;
    println!("value of len {}", len);

    let mut arr: Vec<i32> = arr_and_size.0;

    print_sort(&arr, len);
    sort(&mut arr, 0, len - 1);
    print_sort(&arr, len);
}
