pub fn search(arr: &[u32; 5], fnd_num: u32) -> i32 {
    
    if let Some(v) = arr.iter().find(|&&x| x == fnd_num) {
        return *v as i32;
    }

    -1
}