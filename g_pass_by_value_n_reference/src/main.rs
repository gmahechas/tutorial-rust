fn main() {
    pass_by_value();
    println!("#######");
    pass_by_reference();
}

fn pass_by_value() {
    let x: i32 = 7;

    println!("The initial value of x is: {}", x);
    pass_by_value_one(x);
    println!("The final value of x is: {}", x);
}

fn pass_by_value_one(mut x: i32) {
    x = x + 1;
    println!("The value of x in the inner scope is: {}", x);
}

fn pass_by_reference() {
    let mut x: i32 = 7;

    println!("The initial value of x is: {}", x);
    pass_by_reference_one(&mut x);
    println!("The final value of x is: {}", x);
}

fn pass_by_reference_one(x: &mut i32) {
    *x += 1;
    println!("The value of x in the inner scope is: {}", x);
}
