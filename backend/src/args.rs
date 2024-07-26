use clap::{Parser, Subcommand};

use crate::investments::args::InvestmentTypeCommand;

#[derive(Parser, Debug)]
#[clap(
    author = "Kacper Cienkosz",
    about = "Investment tracker backend app manager.",
    version = "0.1.0"
)]
pub struct InvestTrackerManagerArgs {
    #[clap(subcommand)]
    pub subcommand: ManagerSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum ManagerSubcommand {
    #[clap(name = "investment-type", about = "Manage investment types")]
    InvestmentType(InvestmentTypeCommand),
    RunServer,
}
