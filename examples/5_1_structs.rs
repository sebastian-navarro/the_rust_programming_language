#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count : u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main(){
    let mut user1 = User {
        active: true,
        username: String::from("snavarro"),
        email: String::from("snavarro@mail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anothermail@mail.com");

    println!("User: {:?}", user1);

}