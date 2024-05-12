use models::user::User;

mod discord;
mod jwt;
mod oauth;

pub mod callback;
pub mod cookie;
pub mod login;
pub mod logout;

pub enum AuthKind {
    Human(User),
    Machine(User),
}
