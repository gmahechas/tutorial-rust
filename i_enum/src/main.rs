mod user;
use user::user::User;
use crate::user::user::Role;
use crate::user::user::check_access;

fn main() {
    let user = User::new(
        String::from("tavogus"),
        String::from("tavogus@me.com"),
        32,
        true,
        user::user::Role::User,
        user::user::Website::Facebook(String::from("facebook.com")),
    );

    println!("{}",user);
    println!("user has access: {}", Role::has_access_from_struct(&user.role));
    println!("user has access: {}", check_access(user.role));
}
