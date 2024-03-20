fn main() {
    // borrowing
    let name: String = String::from("John");
    println!("memory address: {:p}", &name);
    let name_copy: String = name;
    println!("memory address: {:p}", &name_copy);
    println!("{}", name_copy);
    // println!("{}", name); // error because name is moved
}
