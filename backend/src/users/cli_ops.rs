use super::cli_args::{CreateUser, DeleteUser, UpdateUser, UserCommand, UserSubcommand};
use super::models::InvestmentUser;
use crate::db::get_pool;
use crate::users::models::NewInvestmentUser;
use diesel::prelude::*;

pub fn handle_user_command(investment_type: UserCommand) {
    let command = investment_type.command;
    match command {
        UserSubcommand::Create(create) => create_investment_type(create),
        UserSubcommand::Update(update) => update_investment_type(update),
        UserSubcommand::Delete(delete) => delete_investment_type(delete),
        UserSubcommand::Show => show_investment_types(),
    }
}

pub fn create_investment_type(user: CreateUser) {
    println!("Creating user: {:?}", user);
    use crate::schema::investment_users::dsl::*;

    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = get_pool(&db_url);
    let mut conn = pool.get().expect("Failed to get a database connection");

    let new_user =
        NewInvestmentUser::new(&user.username, &user.email, &user.password, user.superuser);

    diesel::insert_into(investment_users)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Error saving new user");
}

pub fn update_investment_type(user: UpdateUser) {
    println!("Updating user with id {}: {:?}", user.id, user);
    use crate::schema::investment_types::dsl::*;

    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = get_pool(&db_url);
    let mut conn = pool.get().expect("Failed to get a database connection");

    diesel::update(investment_types.find(user.id))
        .set(type_name.eq(&user.name))
        .execute(&mut conn)
        .expect("Error updating user");
}

pub fn delete_investment_type(user: DeleteUser) {
    println!("Deleting user with id {}", user.id);
    use crate::schema::investment_users::dsl::*;

    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = get_pool(&db_url);
    let mut conn = pool.get().expect("Failed to get a database connection");

    diesel::delete(investment_users.find(user.id))
        .execute(&mut conn)
        .expect("Error deleting users");
}

pub fn show_investment_types() {
    println!("Showing users");
    use crate::schema::investment_users::dsl::*;

    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = get_pool(&db_url);
    let mut conn = pool.get().expect("Failed to get a database connection");

    let results = investment_users
        .load::<InvestmentUser>(&mut conn)
        .expect("Error loading users");

    for user in results {
        println!("{:?}", user);
    }
}
