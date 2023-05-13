use crate::bmi::Bmi;
use crate::clap_parser::Args;
use crate::errors::BmiError;
use crate::height::Height;
use crate::weight::Weight;
//use crate::serde::BmiJson;

use clap::Parser;
//use time::{PrimitiveDateTime};
use std::borrow::Borrow;
//use std::fs::{File, self};
//use std::io::Write;

use inquire::CustomType;

mod bmi;
mod clap_parser;
mod database;
mod errors;
mod height;
mod serde; //framework welches funktionialität bereitstellt, um Daten in Maschinenlesbare Informationen zu konvertieren (Serialize / Deserialize)
mod test;
mod weight;

fn main() {
    env_logger::init();

    log::info!("Programm started");
    //To calculate your BMI with your weight and height as input.
    start_bmi_calculation();
}

fn start_bmi_calculation() {
    let cli = Args::parse();
    if cli.database {
        println!("Printing database");
        /* let contents = fs::read_to_string("bmi_file.txt").expect("Should have read the file.");
        println!("file content: {}", contents);*/

        let database = crate::database::Database::load().expect("Opening database");
        database.print();
    } else {
        println!("Interactive now...");
    }

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
    /*
    let timestamp:PrimitiveDateTime = {
        let now = time::OffsetDateTime::now_local().map_err(DatabaseError::from).unwrap();
        let time = now.time();
        let date = now.date();
        time::PrimitiveDateTime::new(date, time)
    };
    */
    log::debug!("Got the input height, weight and time.");
    //drop(stdin);

    // kg / m^2 = BMI
    let bmi = calculate_bmi(height.borrow(), weight.borrow());
    match bmi {
        Ok(bmi) => {
            println!("Dein BMI: {}", bmi.value()); //Bmi::value(&bmi) funktioniert auch statt bmi.value() //ruft die funktion value von der instanz bmi auf
                                                   /*
                                                   let mut bmi_file = match File::options()
                                                       .create(true)
                                                       .append(true)
                                                       .open("bmi_file.txt")
                                                   {
                                                       Ok(file) => {
                                                           log::debug!("Created/opened a file");
                                                           file
                                                       }
                                                       Err(e) => {
                                                           log::error!("Creating/Opening file failed: {e:?}");
                                                           std::process::exit(1)
                                                       }
                                                   };

                                                   let json_object = make_json_object(height, weight, timestamp);
                                                   let json_string = make_json_string(json_object);

                                                   writeln!(
                                                       &mut bmi_file,
                                                       "Bmi:{}, other info: {}",
                                                       bmi.value(),
                                                       json_string
                                                   )
                                                   .unwrap_or_else(|err| {
                                                       eprintln!("Something went wrong: {err:?}");
                                                       std::process::exit(1)
                                                   });*/

            let entry = database::DatabaseEntry::new(bmi).expect("Creating database entry object");
            let mut database = crate::database::Database::load().expect("Opening database");

            database.add_entry(entry);
            database.store().expect("Storing database");
        }
        Err(_e) => {
            println!("the height value is not ok:");
            std::process::exit(1)
        }
    };
}

// calculates bmi based on height and weight
pub fn calculate_bmi(height: &Height, weight: &Weight) -> Result<Bmi, BmiError> {
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

/*
pub fn make_json_object(height: Height, weight: Weight, time_stamp: PrimitiveDateTime) -> BmiJson {
    BmiJson {
        height,
        weight,
        time_stamp,
    }
}

pub fn make_json_string(json_object: BmiJson) -> String {
    serde_json::to_string_pretty(&json_object).unwrap_or_else(|err| {
        eprintln!("Something went wrong: {err:?}");
        std::process::exit(1)
    })
}*/
