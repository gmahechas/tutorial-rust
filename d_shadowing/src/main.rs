fn main() {
    let x: i32 = 5; // bad practice, becasue we are shadowing x
    {
        let x:i32 = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let x: i32 = x + 2;
    println!("The value of x is: {}", x);

}
