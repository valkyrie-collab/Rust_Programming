fn sum_of_natural_number(number: u32) -> u32 {

    if number == 1 {
        return 1;
    }

    number + sum_of_natural_number(number - 1)
}

fn main() {
    let num: u32 = sum_of_natural_number(10);

    println!("{:?}", num);
}
