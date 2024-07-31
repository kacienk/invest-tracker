#[macro_use]
extern crate diesel;

mod args;
mod auth;
mod db;
mod investments;
mod ops;
mod schema;

use args::{InvestTrackerManagerArgs, ManagerSubcommand};
use clap::Parser;

#[actix_web::main]
async fn main() {
    let manager_command = InvestTrackerManagerArgs::parse();
    env_logger::init();

    match manager_command.subcommand {
        ManagerSubcommand::InvestmentType(investment_type) => {
            investments::ops::handle_investment_type_command(investment_type)
        }
        args::ManagerSubcommand::RunServer => {
            ops::run_server().await.expect("Error running server");
        }
    }
}
