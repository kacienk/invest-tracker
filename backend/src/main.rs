#[macro_use]
extern crate diesel;

mod args;
mod auth;
mod common;
mod db;
mod investment_groups;
mod investment_types;
mod investments;
mod ops;
mod schema;
mod users;

use args::{InvestTrackerManagerArgs, ManagerSubcommand};
use clap::Parser;
use investment_types::cli_ops::handle_investment_type_command;
use users::cli_ops::handle_user_command;

#[actix_web::main]
async fn main() {
    let manager_command = InvestTrackerManagerArgs::parse();
    env_logger::init();

    match manager_command.subcommand {
        ManagerSubcommand::InvestmentType(investment_type) => {
            handle_investment_type_command(investment_type)
        }
        ManagerSubcommand::User(user) => handle_user_command(user),
        ManagerSubcommand::RunServer => {
            ops::run_server().await.expect("Error running server");
        }
    }
}
