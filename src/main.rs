use std::env::args;
use std::io;
use std::str::FromStr;
use std::io::Write;

fn main() {
    let stdin = std::io::stdin();
    print!("Gebe dein Gewicht in kg ein: ");
    let _ = std::io::stdout().flush(); //flush is to really print the stuff

    let mut buffer_weight = String::new();
    stdin.read_line(&mut buffer_weight).unwrap();
    println!(" ");

    let weight = f64::from_str(buffer_weight.trim()).unwrap(); //trim() removed das trailing "enter" also \n was durch das enter drücken mit kommt -> \n kann nicht geparset werden

    println!("Weigth: {weight}");

    let stdin = std::io::stdin();
    print!("Gebe deine Höhe in Meter ein: ");
    let _ = std::io::stdout().flush(); //to really print the stuff

    let mut buffer_height = String::new();
    stdin.read_line(&mut buffer_height).unwrap();
    println!(" ");

    let height = f64::from_str(buffer_height.trim()).unwrap();

    println!("Heigth: {height}");

    let bmi = weight / (height * height);
    println!("BMI: {bmi}");
}


