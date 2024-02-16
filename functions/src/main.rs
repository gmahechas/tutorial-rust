fn add(x: i32, y: i32, z: i32) -> i32 {
    x + y + z
}
fn get_second_number() -> i32 {
    7
}

fn pass_by_reference(my_num1: &mut i32) {
    *my_num1 += 5;
}

fn main() {
    /* Functions */
    print!("####### Functions \n");
    let get_first_number: i32 = {
        3
    };
    let get_third_numer = |num1: i32| -> i32 {
        2 + num1
    };

    let x: i32 = get_first_number;
    let y: i32 = get_second_number();
    let z: i32 = get_third_numer(2);

    let result: i32 = add(x, y, z);
    println!("The sum of x and y is: {}", result);


    /* pass by reference */
    println!("####### Pass by reference");
    let mut my_num1: i32 = 5;
    println!("before: {}", my_num1);
    pass_by_reference(&mut my_num1);
    println!("after: {}", my_num1);
}
