use clap::{Args, Subcommand};

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
    pub id: i32,
    #[clap(short, long)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct DeleteInvestmentType {
    #[clap(short, long)]
    pub id: i32,
}
