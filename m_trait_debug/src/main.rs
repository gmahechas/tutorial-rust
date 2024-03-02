
#[derive(Debug)] // this will add impl std::fmt::Debug (same I have below)
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

fn main() {
   let user = User {
       username: String::from("tavogus"),
       age: 37,
   };
   println!("{} {}", user.username, user.age);
   println!("{:?}", user);
}
