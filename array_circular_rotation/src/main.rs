fn rotate_array(arr: &mut [i32], len: usize, k: usize) -> Vec<i32> {
    let mut new_arr: Vec<i32> = Vec::with_capacity(len);
    let mut len_one: usize =  len - k;
    let len_two: usize = k + 1;
    let mut i: usize = 0;

    while len_one < len {
        new_arr.push(arr[len_one]);
        len_one += 1;
    }

    while i < len_two {
        new_arr.push(arr[i]);
        i += 1;
    }

    new_arr
}

fn main() {
    let mut arr: Vec<i32> = vec![10, 20, 30, 40, 50];
    let len: usize = arr.len();

    let new_arr: Vec<i32> = rotate_array(&mut arr, len, 2);
    println!("{:?}", new_arr);
}
