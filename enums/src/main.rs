use std::fmt;

enum UserType {
    Admin,
    User,
    Guest,
}

impl fmt::Display for UserType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserType::Admin => write!(f, "Administrator"),
            UserType::User => write!(f, "User"),
            UserType::Guest => write!(f, "Guest"),
        }
    }
}

enum Website {
    Facebook(String),
    Twitter(String),
    Google(String),
}

struct User {
    username: String,
    email: String,
    age: i32,
    active: bool,
    user_type: UserType,
    pub website: Website,
}

/* fn check_access(user_type: UserType) -> bool {
    match user_type {
        UserType::Admin => true,
        _ => false
    }
} */

impl User {
    fn has_access_from_struct(&self) -> bool {
        match self.user_type {
            UserType::Admin => true,
            _ => false,
        }
    }
}

impl User {
    pub fn get_user_type(&self) -> &UserType {
        &self.user_type
    }
}

fn main() {
    /* Enums */
    print!("####### Enums \n");
    let user1 = User {
        username: String::from("tavogus"),
        email: String::from("tavogus@tavogus"),
        age: 32,
        active: true,
        user_type: UserType::Admin,
        website: Website::Facebook(String::from("facebook.com")),
    };
    println!(
        "username: {}, email: {}, age: {}, active: {}",
        user1.username, user1.email, user1.age, user1.active
    );
    // println!("has user1 access?: {}", check_access(user1.user_type));
    println!(
        "has user1 access from struct?: {}",
        user1.has_access_from_struct()
    );
    println!("user type: {}", user1.get_user_type());

    let user2 = User {
        user_type: UserType::User,
        ..user1
    };
    println!(
        "username: {}, email: {}, age: {}, active: {}",
        user2.username, user2.email, user2.age, user2.active
    );
    // println!("has user2 access?: {}", check_access(user2.user_type));
    println!(
        "has user2 access from struct?: {}",
        user2.has_access_from_struct()
    );
    println!("user type: {}", user2.get_user_type());
}
