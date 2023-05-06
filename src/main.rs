use std::io::Stdin;
use std::str::FromStr;

use crate::weight::Weight;
use crate::height::Height;
use crate::bmi::Bmi;
use crate::errors::BmiError;
//use crate::errors::InputError;

mod test;
mod bmi;
mod weight;
mod height;
mod errors;


fn main() {


    //To calculate your BMI with your weight and height as input.
    start_bmi_calculation();
}

fn start_bmi_calculation() {
    let stdin = std::io::stdin();

    println!("Bitte Gewicht eingeben (in kg): ");
    let weight: Weight = Weight(get_input(&stdin));

    println!("Bitte Größe eingeben (in meter): ");
    let height: Height = Height(get_input(&stdin));
    //drop(stdin);

    // kg / m^2 = BMI
    let bmi = calculate_bmi(height, weight);
    match bmi {
        Ok(bmi) => println!("Dein BMI: {}", bmi.value()), //Bmi::value(&bmi) funktioniert auch statt bmi.value() //ruft die funktion value von der instanz bmi auf
        Err(_e) => println!("the height value is not ok")
    };
    
}

fn get_input(stdin: &Stdin) -> f64 {
    let mut buffer_height = String::new();
    match stdin.read_line(&mut buffer_height) {
        Ok(_usize) => f64::from_str(buffer_height.trim()).unwrap_or_else(|err| {
            println!("blabla: {}", err);
            get_input(stdin)
        }),
        Err(e) => {
            println!("Input is not the right format or type: {}", e);
            println!("Try again!");
            get_input(stdin)
        }
    }
}

// calculates bmi based on height and weight
pub fn calculate_bmi(height: Height, weight: Weight) -> Result<Bmi, BmiError> {
    if height.0 == 0.0 {
        println!("0");
        return Err(BmiError::HeightIsZero)
    } else if height.0 < 0.0{
        println!("negative");
        return Err(BmiError::HeightIsNegative)
    } else if weight.0 <= 0.0{
        println!("Weight is not ok");
        return Err(BmiError::WeightIsNotOk)
    }
    let bmi = weight.0 / (f64::powf(height.0, 2.0));
    Ok(Bmi::new(bmi)) //kreiert ein neue Bmi instanz
}
