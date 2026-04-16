fn main() {
    let x: u32 = 4;

    fn equal_to_x(z: u32) -> bool {z == x}

    let y: u32 = 4;

    assert!(equal_to_x(y));
}
