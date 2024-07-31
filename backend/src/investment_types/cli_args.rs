use clap::{Args, Subcommand};
use uuid::Uuid;

#[derive(Debug, Args)]
pub struct InvestmentTypeCommand {
    #[clap(subcommand)]
    pub command: InvestmentTypeSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum InvestmentTypeSubcommand {
    Create(CreateInvestmentType),
    Update(UpdateInvestmentType),
    Delete(DeleteInvestmentType),
    Show,
}

#[derive(Debug, Args)]
pub struct CreateInvestmentType {
    #[clap(short, long)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct UpdateInvestmentType {
    #[clap(short, long)]
    pub id: Uuid,
    #[clap(short, long)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct DeleteInvestmentType {
    #[clap(short, long)]
    pub id: Uuid,
}
