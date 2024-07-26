use crate::db::establish_connection;
use crate::investments::args::{
    CreateInvestmentType, DeleteInvestmentType, InvestmentTypeCommand, InvestmentTypeSubcommand,
    UpdateInvestmentType,
};
use crate::investments::models::investment_type::{InvestmentType, NewInvestmentType};
use diesel::prelude::*;

pub fn handle_investment_type_command(investment_type: InvestmentTypeCommand) {
    let command = investment_type.command;
    match command {
        InvestmentTypeSubcommand::Create(create) => create_investment_type(create),
        InvestmentTypeSubcommand::Update(update) => update_investment_type(update),
        InvestmentTypeSubcommand::Delete(delete) => delete_investment_type(delete),
        InvestmentTypeSubcommand::Show => show_investment_types(),
    }
}

pub fn create_investment_type(investment_type: CreateInvestmentType) {
    println!("Creating investment type: {:?}", investment_type);
    use crate::schema::investment_types::dsl::*;

    let connection = establish_connection();
    let new_investment_type = NewInvestmentType {
        name: &investment_type.name,
    };

    diesel::insert_into(investment_types)
        .values(&new_investment_type)
        .execute(&connection)
        .expect("Error saving new investment type");
}

pub fn update_investment_type(investment_type: UpdateInvestmentType) {
    println!(
        "Updating investment type with id {}: {:?}",
        investment_type.id, investment_type
    );
    use crate::schema::investment_types::dsl::*;

    let connection = establish_connection();
    diesel::update(investment_types.find(investment_type.id))
        .set(type_name.eq(&investment_type.name))
        .execute(&connection)
        .expect("Error updating investment type");
}

pub fn delete_investment_type(investment_type: DeleteInvestmentType) {
    println!("Deleting investment type with id {}", investment_type.id);
    use crate::schema::investment_types::dsl::*;

    let connection = establish_connection();
    diesel::delete(investment_types.find(investment_type.id))
        .execute(&connection)
        .expect("Error deleting investment type");
}

pub fn show_investment_types() {
    println!("Showing investment types");
    use crate::schema::investment_types::dsl::*;

    let connection = establish_connection();
    let results = investment_types
        .load::<InvestmentType>(&connection)
        .expect("Error loading investment types");

    for investment_type in results {
        println!("{:?}", investment_type);
    }
}