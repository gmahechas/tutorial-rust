/*
####### escalar types #######
Integers
i8, i16, i32, i64, i128, and isize (integers with sign) positive and negative values
u8, u16, u32, u64, u128, and usize (integers without sign) only positive values

Floating-Point
f32, f64 (floating-point numbers)

Boolean
bool (true or false)

Character
char (single Unicode character)

####### compound types #######
Tuple
(i32, f64, u8)

Arrays
[3, 4, 5]
[i32; 3] (array with 3 elements of type i32)

Pointers
&T (for a reference to a value of type T)
&mut T (for a mutable reference to a value of type T)
Box (allocated on the heap)
Rc (counter reference)
Arc (atomic counter reference)
Mutex and RwLock (concurrent read and write locks)
*/

fn main() {
    /* Constants */
    print!("####### Constants \n");
    const MAX_POINTS: u32 = 100_000;
    let string_formatted = format!("The value of MAX_POINTS is: {}", MAX_POINTS);
    println!("{}", string_formatted);

    /* Scope */
    print!("####### Scope \n");
    let x: i32 = 4;
    let x: i32 = x + 1;
    {
        let x = x * 2;  // x is now 12
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    /* tuple */
    print!("####### Tuples \n");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}
