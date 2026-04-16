use std::sync::{Mutex, MutexGuard};

const MAX: usize = 5;
static HEAD: Mutex<usize> = Mutex::new(0);
static FOOT: Mutex<usize> = Mutex::new(0);
static COUNT: Mutex<usize> = Mutex::new(0);

fn push(arr: &mut [i32; MAX], val: i32) {
    let mut index: usize = *HEAD.lock().unwrap();
    let mut count: usize = *COUNT.lock().unwrap();
    count += 1;

    if count > MAX {
        return;
    }

    index = index % MAX;
    
    if index >= MAX {
        index = 0
    }
    
    arr[index] = val;
    index += 1;

    {
        let mut update_index: MutexGuard<'_, usize> = HEAD.lock().unwrap();
        *update_index = index;

        let mut update_count: MutexGuard<'_, usize> = COUNT.lock().unwrap();
        *update_count = count;
    }
}

fn pop(arr: &mut [i32; MAX]) -> i32 {
    let mut index: usize = *FOOT.lock().unwrap();
    let mut count: usize = *COUNT.lock().unwrap();
    let old_index: usize;

    if count == 0 {
        return -1;
    }

    count -= 1;

    if count < 1 {
        return -1;
    }

    index = index % MAX;
    old_index = index;

    if index >= MAX {
        index = 0;
    }

    index += 1;

    {
        let mut update_index: MutexGuard<'_, usize> = FOOT.lock().unwrap();
        *update_index = index;

        let mut update_count: MutexGuard<'_, usize> = COUNT.lock().unwrap();
        *update_count = count;
    }

    arr[old_index]
}

fn main() {
    let mut arr: [i32; MAX] = [0; MAX];
    push(&mut arr, 5);
    push(&mut arr, 3);
    push(&mut arr, 6);
    println!("{}", pop(&mut arr));
    println!("{}", pop(&mut arr));
    println!("{}", pop(&mut arr));
    println!("{}", pop(&mut arr));
}
