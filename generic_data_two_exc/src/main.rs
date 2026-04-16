struct Point<T> {
    x: T,
    y: T
}

struct PointTwo<T, U> {
    x: T,
    y: U
}

enum Value<T> {
    val(T)
}

impl<T> Point<T> {
    fn x_method(&self) -> &T {
        &self.x
    }
}

impl Point<i32> {
    fn y_method(&self) -> &i32 {
          &self.y
    }
}

impl<T, U> PointTwo<T, U> {
    fn mixup<V, W>(self, other: PointTwo<V, W>) -> PointTwo<T, W> {
        PointTwo {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let integer: Point<i32> = Point { x: 32, y: 67 };
    let float: Point<f32> = Point { x: 7.34, y: 8.90 };
    let value: Value<String> = Value::val(String::from("value"));
    println!("{}", integer.x_method());

    let p1: PointTwo<i32, f32> = PointTwo { x: 5, y: 10.4 };
    let p2: PointTwo<&str, char> = PointTwo { x:"Hello", y: 'c' };
    let p3: PointTwo<i32, char> = p1.mixup(p2);
    println!(" x:{} y:{} ", p3.x, p3.y)
}
