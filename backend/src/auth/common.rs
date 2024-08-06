use crate::users::models::NewInvestmentUser;

use super::services::password_service::{HashedPassword, PasswordService};

pub fn new_user(
    username: &String,
    email: &String,
    passwd: &String,
    superuser: bool,
) -> NewInvestmentUser {
    let HashedPassword { hash, salt } = PasswordService::hash_password(passwd);

    NewInvestmentUser {
        username: username.clone(),
        email: email.clone(),
        password: hash,
        salt,
        superuser: superuser.clone(),
    }
}
