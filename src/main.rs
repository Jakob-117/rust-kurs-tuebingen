use crate::bmi::Bmi;
use crate::errors::BmiError;
use crate::height::Height;
use crate::weight::Weight;
//use crate::errors::InputError;

use std::fs::File;
use std::io::Write;

use inquire::CustomType;

mod bmi;
mod errors;
mod height;
mod test;
mod weight;

fn main() {
    env_logger::init();

    log::info!("Programm started");
    //To calculate your BMI with your weight and height as input.
    start_bmi_calculation();
}

fn start_bmi_calculation() {
    let weight: Weight = CustomType::<f64>::new("Your weight in kg please")
        .with_formatter(&|i| format!("kg{:.2}", i))
        .with_error_message("Can I get a water please?")
        .with_help_message("Type the amount of weight in this format 00.0")
        .prompt()
        .map(Weight)
        .unwrap_or_else(|err| {
            eprintln!("Something went wrong: {err:?}");
            std::process::exit(1)
        });

    let height: Height = CustomType::<f64>::new("Your height in meter please")
        .with_formatter(&|i| format!("m{:.2}", i))
        .with_error_message("Can I get a water please?")
        .with_help_message("Type your height in this format 0.00")
        .prompt()
        .map(Height)
        .unwrap_or_else(|err| {
            eprintln!("Something went wrong: {err:?}");
            std::process::exit(1)
        });

    log::debug!("Got the input height and weight.");
    //drop(stdin);

    // kg / m^2 = BMI
    let bmi = calculate_bmi(height, weight);
    match bmi {
        Ok(bmi) => {
            println!("Dein BMI: {}", bmi.value()); //Bmi::value(&bmi) funktioniert auch statt bmi.value() //ruft die funktion value von der instanz bmi auf
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
            writeln!(&mut bmi_file, "Bmi:{}", bmi.value()).unwrap();
        }
        Err(_e) => {
            println!("the height value is not ok:");
            std::process::exit(1)
        }
    };
}

// calculates bmi based on height and weight
pub fn calculate_bmi(height: Height, weight: Weight) -> Result<Bmi, BmiError> {
    if height.0 == 0.0 {
        println!("0");
        return Err(BmiError::HeightIsZero);
    } else if height.0 < 0.0 {
        println!("negative");
        return Err(BmiError::HeightIsNegative);
    } else if weight.0 <= 0.0 {
        println!("Weight is not ok");
        return Err(BmiError::WeightIsNotOk);
    }
    let bmi = weight.0 / (f64::powf(height.0, 2.0));
    Ok(Bmi::new(bmi)) //kreiert ein neue Bmi instanz
}
