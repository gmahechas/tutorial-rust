fn main() {
    const MAX_POINTS: u32 = 100000; // inmutable, you should specify the type
    let string_formatted: String = format!("The value of MAX_POINTS is: {}", MAX_POINTS); // inmutable
    println!("Result: {}", string_formatted);

    let mut x: i32 = 5; // mutable
    x = x + 7;
    println!("The value of x is: {}", x);

    static Y: i32 = 7; // inmutable
    println!("The value of y is: {}", Y);

    unsafe {
        static mut Z: i32 = 7; // mutable, only works in unsafe blocks
        Z = Z + 1;
        println!("The value of z is: {}", Z);
    }

    let tup: (i32, f64, i32, bool) = (500, 6.4, 1, true); // tuple
    println!("The tup is: {:?}", tup);
}
       
