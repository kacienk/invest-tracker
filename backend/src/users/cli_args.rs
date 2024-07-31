use clap::{Args, Subcommand};
use uuid::Uuid;

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    Create(CreateUser),
    Update(UpdateUser),
    Delete(DeleteUser),
    Show,
}

#[derive(Debug, Args)]
pub struct CreateUser {
    #[clap(short, long)]
    pub username: String,
    #[clap(short, long)]
    pub email: String,
    #[clap(short, long)]
    pub password: String,
    #[clap(short, long)]
    pub superuser: bool,
}

#[derive(Debug, Args)]
pub struct UpdateUser {
    #[clap(short, long)]
    pub id: Uuid,
    #[clap(short, long)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct DeleteUser {
    #[clap(short, long)]
    pub id: Uuid,
}
