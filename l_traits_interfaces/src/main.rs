trait Speaks {
    fn speak(&self) -> String;
    fn language(&self) -> String {
        String::from("default language")
    }
}

struct Human;
struct Animal;

impl Speaks for Human {
    fn speak(&self) -> String {
        String::from("Hello")
    }

    fn language(&self) -> String {
        String::from("English")
    }
}

impl Speaks for Animal {
    fn speak(&self) -> String {
        String::from("Woof")
    }
}

// duck typing
fn speak(speaks: &impl Speaks) {
    println!("{}", speaks.speak());
}

fn main() {
    let human: Human = Human;
    let animal: Animal = Animal;

    println!("{}", human.speak());
    println!("{}", animal.speak());

    println!("{}", human.language());
    println!("{}", animal.language());

    speak(&human);
    speak(&animal);
}
