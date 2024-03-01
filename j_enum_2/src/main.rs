fn main() {
    enum_one();
    enum_two();
}

fn enum_one() {
    let name: Option<String> = None;

    match name {
        None => println!("none"),
        Some(ref name) => println!("name: {}", name),
    }
}

fn enum_two() {
    struct Person {
        name: String,
        age: Option<i32>,
    }

    impl Person {
        fn get_age(&self) -> i32 {
            if self.age.is_none() {
                0
            } else {
                self.age.unwrap_or_default()
            }
        }
    }


    let person = Person {
        name: String::from("tavogus"),
        age: None,
    };

    println!("person age: {}", person.get_age());
}