fn get_largest(list: &[i32]) -> i32 {
    let mut largest: i32 = list[0];

    for number in list {
        
        if *number > largest {
            largest = *number;
        }

    }

    largest
}

fn check_arr(list: [i32; 5]) {
    println!("the value {}", list[0]);
}

fn get_largest_g<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest: T = list[0];

    for number in list {
        
        if *number > largest {
            largest = *number;
        }

    }

    largest

}

fn main() {
    let number_list: Vec<i32> = vec![34, 50, 100, 65];
    
    let mut largest: i32 = number_list[0];

    for number in number_list {
        
        if number > largest {
            largest = number;
        }

    }

    println!("The number is {}", largest);

    let number_list: Vec<i32> = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    largest = number_list[0];

    for i in number_list {

        if i > largest {
            largest = i;
        }

    }

    let number_list: Vec<i32> = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let new_list: [i32; 5] = [9, 0, 8, 7, 6];

    println!("The second largest number: {}", largest);
    println!("The largest number: {}", get_largest(&number_list));
    check_arr(new_list);
    println!("The forith largest number: {}", get_largest_g(&number_list));

    let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {} ", get_largest_g(&char_list))
}
