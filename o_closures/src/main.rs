fn main() {
    lambda();
    println!("#######");
    closure();
    println!("#######");
    closure_two();
}

fn lambda () {
    let sum: fn(i32, i32) -> i32 = |num1: i32, num2: i32|  num1 + num2;
    println!("{}", sum(5, 2));
}

fn closure() {
    let counter: i32 = 0;
    println!("counter before closure: {}", counter);
    
    let mut other_counter: &i32 = &counter;
    println!("other_counter before closure: {}", other_counter);
    
    other_counter = &7;
    
    let increase = || {
        println!("counter inside closure: {}", counter);
        println!("other_counter inside closure: {}", other_counter);
    };

    increase();
    
    println!("counter after closure: {}", counter);
    println!("other_counter after closure: {}", other_counter);
}

fn closure_two() {
    let mut counter: i32 = 0;
    println!("counter before closure: {}", counter);
    
    let other_counter: &mut i32 = &mut counter;
    println!("other_counter before closure: {}", other_counter);
    
    *other_counter = 7;
    
    //                         move, this will take all as value, other_counter is used as value
    //                          |
    let increase = || {
        // println!("counter inside closure: {}", counter); // can not print this because it already borrowed
        println!("other_counter inside closure: {}", other_counter);
    };
    
    println!("other_counter after closure: {}", other_counter);
    increase();
}
