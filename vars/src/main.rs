fn main() {
    /* Variables */
    print!("####### Variables \n");
    let x: i32 = 7;
    println!("The value of x is: {}", x);

    /* Mutability */
    print!("####### Mutability \n");
    let mut y = 6;
    println!("The value of y is: {}", y);
    y = 7;
    println!("Now the value of y is: {}", y);

    /* Shadowing */
    print!("####### Shadowing \n");
    let z: i32 = 8;
    let z: i32 = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {}", z);
    }
    println!("The value of z is: {}", z);
}
