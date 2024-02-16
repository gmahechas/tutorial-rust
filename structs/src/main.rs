struct User {
    username: String,
    email: String,
    age: i32,
    active: bool,
}

/* struct impl */
impl User {
    fn print(&self) {
        println!("username: {}, email: {}, age: {}, active: {}", self.username, self.email, self.age, self.active);
    }
}

fn main() {
    let user1: User = new_user(String::from("tavogus"), String::from("tavogus@me.com"), 32, true);
    user1.print();

    let user2: User = User {
        active: false,
        ..user1
    };
    user2.print();

    /* Tuple struct */
    struct MyColor(i32, i32, i32);
    let color = MyColor(255, 0, 0);
    println!("R: {}, G: {}, B: {}", color.0, color.1, color.2);
}

/* short hand init */
fn new_user(username: String, email: String, age: i32, active: bool) -> User {
    User {
        username,
        email,
        age,
        active,
    }
}
