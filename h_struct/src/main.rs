mod user;
use user::user::User;

fn main() {
    let mut user = User::new(
        String::from("tavogus"),
        String::from("tavogus@me.com"),
        32,
        true,
    );
    user.print();
    let mut new_email: String = user.get_email();
    println!("new email: {}", new_email);
    new_email = String::from("analuna@me.com");
    println!("new email: {}", new_email);
    user.print();

    user.set_email(new_email);
    user.print();

    let user2: User = User::new(
        user.username.clone(),
        user.get_email(),
        user.age,
        user.active,
    );
    user2.print();

    /* Tuple struct */
    struct MyColor(i32, i32, i32);
    let color = MyColor(255, 0, 0);
    println!("R: {}, G: {}, B: {}", color.0, color.1, color.2);
}
