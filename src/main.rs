use std::env::args;
use std::io::{self, Stdin};
use std::str::FromStr;
use std::io::Write;


fn main() {
    let stdin = std::io::stdin();
    get_weight(stdin)
    
}


fn get_weight(input: Stdin){
    
    print!("Gebe dein Gewicht in kg ein: ");
    let _ = std::io::stdout().flush(); //flush is to really print the stuff

    let mut buffer_weight = String::new();
    input.read_line(&mut buffer_weight).unwrap();
    println!(" ");

    let weight = f64::from_str(buffer_weight.trim()).unwrap(); //trim() removed das trailing "enter" also \n was durch das enter drücken mit kommt -> \n kann nicht geparset werden
    

    println!("Weigth: {weight}");
    get_height(input ,weight);
    
}


fn get_height(input: Stdin, weight: f64){

    print!("Gebe deine Höhe in Meter ein: ");
    let _ = std::io::stdout().flush(); //to really print the stuff

    let mut buffer_height = String::new();
    input.read_line(&mut buffer_height).unwrap();
    println!(" ");

    let height = f64::from_str(buffer_height.trim()).unwrap();

    println!("Heigth: {height}");
    bmi_calc(height, weight);
}



fn bmi_calc(height: f64, weight: f64) -> f64{
    let bmi = weight / (height * height);
    println!("BMI: {bmi}");
    bmi
}
