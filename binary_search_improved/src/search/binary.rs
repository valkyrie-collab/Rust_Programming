pub fn search(arr: &[u32; 5], mut low: usize, mut high: usize, fnd_num: u32) -> i32 {
    let mut mid: usize;

    while low <= high {
        mid = low + (high - low) / 2;

        if arr[mid] == fnd_num {
            return arr[mid] as i32;
        } else if arr[mid] > fnd_num {
            high = mid - 1;
        } else {
            low = mid + 1;
        }

    }

    -1
}