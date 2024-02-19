fn main() {
    let x: i32 = 5; // bad practice, becasue we are shadowing x
    println!("The value of x is: {}", x);

    let x: i32 = x + 2;
    println!("The value of x is: {}", x);
}
