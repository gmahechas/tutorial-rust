use std::fmt;

/* ####### user ####### */
pub struct User {
    pub username: String,
    pub email: String,
    pub age: i32,
    pub active: bool,
    pub role: Role,
    pub website: Website
}

impl User {
    pub fn new(username: String, email: String, age: i32, active: bool, role: Role, website: Website) -> User {
        User {
            username,
            email,
            age,
            active,
            role,
            website,
        }
    }

}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "username: {}, email: {}, age: {}, active: {}, role: {}, website: {}", self.username, self.email, self.age, self.active, self.role, self.website)
    }
}

/* ####### role ####### */
pub enum Role {
    Admin,
    User
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Role::Admin => "Admin",
            Role::User => "User",
        })
    }
}

impl Role {
    pub fn has_access_from_struct(&self) -> bool {
        match self {
            Role::Admin => true,
            Role::User => false
        }
    }
}

pub fn check_access(role: Role) -> bool {
    match role {
        Role::Admin => true,
        Role::User => false
    }
}

/* ####### website ####### */
pub enum Website {
    Github(String),
    Facebook(String),
    Twitter(String)
}

impl fmt::Display for Website {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Website::Github(url) => url,
            Website::Facebook(url) => url,
            Website::Twitter(url) => url
        })
    }
}
/* ####### */
