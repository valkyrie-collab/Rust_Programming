// unsafe fn dangerous() {}

use core::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len: usize = slice.len();
    let ptr: *mut i32 = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid)
        )
    }
}

fn main() {
    // let mut num: i32 = 5;

    // let r1: *const i32 = &num as *const i32;
    // let r2: *mut i32 = &mut num as *mut i32;

    // unsafe {
    //     println!("r1: {}", *r1);
    //     println!("r2: {}", *r2);

    //     dangerous();
    // }

    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let r: &mut [i32] = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

}
