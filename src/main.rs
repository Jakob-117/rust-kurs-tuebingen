use crate::clap_parser::Args;
use crate::errors::BmiError;
use crate::height::Height;
use crate::weight::Weight;
use crate::{bmi::Bmi, database::Database};

use clap::Parser;
use inquire::CustomType;
use web::*;

mod bmi;
mod clap_parser;
mod database;
mod errors;
mod height;
mod serde; //framework welches funktionialität bereitstellt, um Daten in Maschinenlesbare Informationen zu konvertieren (Serialize / Deserialize)
mod test;
mod web;
mod weight;

#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!("Programm started");
    //To calculate your BMI with your weight and height as input.

    let cli = Args::parse();
    if cli.database {
        println!("Printing database");
        let database = crate::database::Database::load().expect("Opening database");
        database.print();
    } else if cli.webui {
        println!("Starting webserver");
        let database = Database::load().expect("Opening database");
        let database = std::sync::Arc::new(tokio::sync::RwLock::new(database));
        router(axum::extract::State(database)).await;
    } else {
        println!("Interactive now");

        let weight: Weight = CustomType::<f64>::new("Your weight in kg please")
            .with_formatter(&|i| format!("kg{:.2}", i))
            .with_error_message("Can I get a water please?")
            .with_help_message("Type the amount of weight in this format 00.0")
            .prompt()
            .map(Weight::new)
            .unwrap_or_else(|err| {
                eprintln!("Something went wrong: {err:?}");
                std::process::exit(1)
            });

        let height: Height = CustomType::<f64>::new("Your height in meter please")
            .with_formatter(&|i| format!("m{:.2}", i))
            .with_error_message("Can I get a water please?")
            .with_help_message("Type your height in this format 0.00")
            .prompt()
            .map(Height::new)
            .unwrap_or_else(|err| {
                eprintln!("Something went wrong: {err:?}");
                std::process::exit(1)
            });

        log::debug!("Got the input height, weight and time.");
        println!("got the weight and height");

        // kg / m^2 = BMI
        let bmi = calculate_bmi(height, weight);
        match bmi {
            Ok(bmi) => {
                println!("Dein BMI: {}", bmi.value()); //Bmi::value(&bmi) funktioniert auch statt bmi.value() //ruft die funktion value von der instanz bmi auf

                let entry =
                    database::DatabaseEntry::new(bmi).expect("Creating database entry object");
                let mut database = crate::database::Database::load().expect("Opening database");

                database.add_entry(entry);
                database.store().expect("Storing database");
            }
            Err(e) => {
                println!("Error while calculating: {e:?}");
            }
        };
    }
}

// calculates bmi based on height and weight
pub fn calculate_bmi(height: Height, weight: Weight) -> Result<Bmi, BmiError> {
    if height.value() == 0.0 {
        println!("0");
        return Err(BmiError::HeightIsZero);
    } else if height.value() < 0.0 {
        println!("negative");
        return Err(BmiError::HeightIsNegative);
    } else if weight.value() <= 0.0 {
        println!("Weight is not ok");
        return Err(BmiError::WeightIsNotOk);
    }
    let bmi = weight.value() / (f64::powf(height.value(), 2.0));
    Ok(Bmi::new(bmi)) //kreiert ein neue Bmi instanz
}
