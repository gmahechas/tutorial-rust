#[derive(Debug)] // this will add impl std::fmt::Debug (same I have below)
#[derive(Default)]
struct User {
    username: String,
    age: i32,
}

/* impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("username", &self.username)
            .field("age", &self.age)
            .finish()
    }
} */

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "username: {}, age: {}", self.username, self.age)
    }
}

fn main() {
   let user: User = User {
       username: String::from("tavogus"),
       age: 37,
   };

   
   println!("{} {}", user.username, user.age);
   println!("{:?}", user); // works because of impl std::fmt::Debug or Debug trait
   println!("{}", user);  // works because of impl std::fmt::Display, you can not use #[derive(std::fmt::Display)] on struct
   
   let user2: User = User::default();
   println!("{}", user2);
}
