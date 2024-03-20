fn main() {
    // if
    let number: i32 = 3;
    if number < 5 {
        println!("condition was true");
    } else if number == 5 {
        println!("condition was false");
    } else {
        println!("condition was false");
    }

    // ternary
    let result: i32 = if number > 5 { 7 } else { 2 };
    println!("result: {}", result);

    // loop
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result: {}", result);

    // while
    let mut number: i32 = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    // switch
    let number: i32 = 5;
    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    // if-let
    let some_option_value: Option<i32> = Some(5);
    if let Some(x) = some_option_value {
        println!("The value of x is: {}", x);
    }

    // while-let
    let mut some_option_value: Option<i32> = Some(5);
    while let Some(x) = some_option_value {
        println!("The value of x is: {}", x);
        some_option_value = None;
    }

    // let-else
    let some_option_value: Option<i32> = None;
    let x: i32 = if let Some(x) = some_option_value {
        x
    } else {
        0
    };
    println!("The value of x is: {}", x);

    // labeled blocks, return with break in a labeled block
    let some_number: Option<i32> = None;
    let result: i32 = 'block1: {
        let inner_result: i32 = 'inner: {
            break 'inner some_number.unwrap_or(0);
        };
        break 'block1 inner_result * 2;
    };
    println!("Result: {}", result);
    
    
}
