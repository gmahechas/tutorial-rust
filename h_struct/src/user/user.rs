#[derive(Clone)]
pub struct User {
    pub username: String,
    email: String,
    pub age: i32,
    pub active: bool,
}

impl User {
    pub fn new(username: String, email: String, age: i32, active: bool) -> User {
        User {
            username,
            email,
            age,
            active,
        }
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    /*     pub fn get_email(&self) -> &String {
        &self.email
    } */

    pub fn set_email(&mut self, new_email: String) {
        self.email = new_email
    }

		pub fn print(&self) {
			println!("username: {}, email: {}, age: {}, active: {}", self.username, self.email, self.age, self.active);
	}
}
