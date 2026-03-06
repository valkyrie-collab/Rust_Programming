mod outermost {
    pub fn middle_function() {
        println!("middle function");
    }

    fn middle_secret_function() {
        println!("middle secret function");
    }

    pub mod inside {
        pub fn inner_function() {
            println!("inner function");
        }

        fn secret_function() {
            println!("secret function");
        }
    }
}

fn try_me() {
    outermost::middle_function();
    // outermost::middle_secret_function();
    outermost::inside::inner_function();
    // outermost::inside::secret_function;
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
