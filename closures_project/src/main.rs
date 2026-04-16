use std::thread;
use std::time::Duration;

struct Cacher<T: Fn(u32) -> u32> {
    calculation: T,
    value: Option<u32>,
}

impl<T: Fn(u32) -> u32> Cacher<T> {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v: u32 = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generated_workout(intensity: u32, random_number: u32) {
    // let expensive_result: u32 = simulated_expensive_calculation(intensity);
    // let mut num: u32 = 0;
    // let expensive_result: fn(u32) -> u32 = |num| -> u32 {
    //     println!("calculting slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    let mut expensive_result: Cacher<fn(u32) -> u32> = Cacher::new( |num| {
        println!("calculting slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next do {} situps!", expensive_result.value(intensity));
    } else {

        if random_number == 3 {
            println!("Take a break today! Remember to say hydrated!");
        } else {
            println!("Today, run for {} minutes", expensive_result.value(intensity));
        }

    }

}

fn main() {
    // println!("{}", simulated_expensive_calculation(56));
    let simulated_user_specified_value: u32 = 10;
    let simulated_random_number: u32 = 7;

    generated_workout(simulated_user_specified_value, simulated_random_number);
}
