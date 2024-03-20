fn main() {
    // lifetime
    let x: i32 = 5;
    do_something(&x, &x);
    println!("{}", x);
}


fn do_something<'a, 'b>(parama: &'a i32, paramb: &'b i32) -> i32 {
    3
}