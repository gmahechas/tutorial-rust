use std::char;

fn main() {
    // integers
    let _a: i8 = -7;
    let _b: u8 = 7;

    let _c: i16 = -7;
    let _d: u16 = 7;

    let _e: i32 = -7;
    let _f: u32 = 7;

    let _g: i64 = -7;
    let _h: u64 = 7;

    let _i: i128 = -7;
    let _j: u128 = 7;

    let _k: isize = -7; // depends on the architecture, e.g 32bit = -2147483648, 64bit = -9223372036854775808
    let _l: usize = 7;

    // floats
    let _m: f32 = 7.0;
    let _n: f64 = 7.0;

    // booleans
    let _o: bool = true;

    // characters
    let _p: char = 'a';
    let _heart_eyed_cat: char = '😻';

    // tuples
    let _q: (i32, f64, char) = (7, 7.0, 'a');
    let (_r, _s, _t) = _q;
    println!("{:?}", _q);
    println!("first element in tuple: {}", _q.0);

    // arrays, they have a fixed length, the data type must be the same
    let _u: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    let _v: [i32; 7] = [1; 7];

    // vector, they can grow or shrink, data type must be the same
    let _w: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];

    println!("{:?}", _u);

    // integers literals
    let _decimal: i32 = 98_222;
    let _hex: i32 = 0xff;
    let _binary: i32 = 0b1111_0000;
    let _octal: i32 = 0o77;
    let _byte: u8 = b'A';

    // strings
    let mut _name: &'static str = "tavogus"; // static lifetime and mutable
    let _a_string: &str = "hello"; // dynamic lifetime and immutable
    let _b_string: String = String::from("hello"); // dynamic lifetime
    println!("{}", _a_string);

    // string slide
    let _c_string: &str = &_b_string[0..2];
    println!("{}", _c_string);
    
}
