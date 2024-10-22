pub mod hello_user;
pub use hello_user::*;

pub mod home;
pub use home::*;

pub mod todo;
pub use todo::*;

pub mod create_user;
pub use create_user::*;

pub fn logging(path: &str) {
    println!("{}", path);
}