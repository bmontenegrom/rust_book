fn main() {
    let user1 = User{
        active: true,
        username: String::from("someusername"),
        email: String::from("someobe@exaple.com"),
        sing_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };



}

struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User { active: true,
        username,
        email, 
        sing_in_count: 1, 
    }
}