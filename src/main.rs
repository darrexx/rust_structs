fn main() {
    let user = build_user(String::from("example@example.com"), String::from("exampleuser"));
    println!("{:?}", user);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}