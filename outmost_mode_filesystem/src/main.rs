pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("nested modules");
            }
        }
    }
}

use a::series::of;

enum TrafficLight {
    Red,
    Yellow,
    Green
}

enum NewTrafficLight {
    Reds,
    Yellows,
    Greens
}

use TrafficLight::{Red, Yellow};
use NewTrafficLight::*;

fn main() {
    of::nested_modules();
    let red: TrafficLight = Red;
    let yellow: TrafficLight = Yellow;
    let green: TrafficLight = TrafficLight::Green;

    let red: NewTrafficLight = Reds;
    let yellow: NewTrafficLight = Yellows;
    let green: NewTrafficLight = Greens;
}