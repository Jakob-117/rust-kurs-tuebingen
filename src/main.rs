use std::env::args;
use std::io::Write;
use std::io::{self, Stdin};
use std::str::FromStr;

fn main() {
    let stdin = std::io::stdin();
    let weight = get_weight(&stdin);
    println!("Weigth: {weight}");

    let height = get_height(&stdin);
    println!("Heigth: {height}");
    let bmi = bmi_calc(height, weight);
    println!("BMI: {bmi}");
}

fn get_weight(input: &Stdin) -> f64 {
    print!("Gebe dein Gewicht in kg ein: ");
    let _ = std::io::stdout().flush(); //flush is to really print the stuff

    let mut buffer_weight = String::new();
    input.read_line(&mut buffer_weight).unwrap();
    println!(" ");

    let weight = f64::from_str(buffer_weight.trim()).unwrap(); //trim() removed das trailing "enter" also \n was durch das enter drücken mit kommt -> \n kann nicht geparset werden
    weight
}

fn get_height(input: &Stdin) -> f64 {
    print!("Gebe deine Höhe in Meter ein: ");
    let _ = std::io::stdout().flush(); //to really print the stuff

    let mut buffer_height = String::new();
    input.read_line(&mut buffer_height).unwrap();
    println!(" ");

    let height = f64::from_str(buffer_height.trim()).unwrap();
    height
}

fn bmi_calc(height: f64, weight: f64) -> f64 {
    let bmi = weight / (height * height);
    bmi
}
