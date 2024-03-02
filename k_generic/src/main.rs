
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let point_int: Point<i32> = Point { x: 1, y: 7 };
    println!("The point is: {:?}", point_int.x);
    println!("The point is: {:?}", point_int.y);

    let point_float: Point<f64> = Point { x: 0.0, y: 7.0 };
    println!("The point is: {:?}", point_float.x);
    println!("The point is: {:?}", point_float.y);

    println!("The area is: {}", calculate_area(point_int));
    println!("The area is: {}", calculate_area(point_float));
}


fn calculate_area<T: std::ops::Mul<Output = T>>(point: Point<T>) -> T {
    point.x * point.y
}
