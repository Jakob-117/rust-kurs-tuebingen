use std::f32;
use std::io;
use std::io::Stdin;
use std::io::Write;
use std::str::FromStr;

struct Weight(f64); //8 Byte groß, dank Zero cost abstractions

struct Height(f64); //geht auch so, damit kann man mit Height.0 drauf zu greifen -> um .0 weg zu bekommen, muss man den Display trait implementieren (anstrengend)

struct BMI(f64);
/*
impl Drop for BMI{ //destructor for structs -> normally generated from the compiler -> braucht man praktisch nie
    fn drop(&mut self){ //kein quatsch in diesem Block machen, man kann mit &mut noch innerhalb was ändern, aber wird dann sofort gedroppt
        todo!()
    }
}*/

fn main() {
    let mut stdin = std::io::stdin();

    println!("Bitte Gewicht eingeben (in kg): ");
    let weight: Weight = Weight(get_input(&stdin));

    println!("Bitte Größe eingeben (in meter): ");
    let height: Height = Height(get_input(&stdin));
    drop(stdin);

    // kg / m^2 = BMI
    let bmi = calculate_bmi(height, weight);

    println!("Dein BMI: {}", bmi.0);
}

fn get_input(stdin: &Stdin) -> f64 {
    let mut buffer_height = String::new();
    match stdin.read_line(&mut buffer_height) {
        Ok(usize) => f64::from_str(buffer_height.trim()).unwrap(),
        Err(e) => panic!("There is something wrong: {}", e),
    }
}

// calculates bmi based on height and weight
fn calculate_bmi(height: Height, weight: Weight) -> BMI {
    let bmi = weight.0 / (f64::powf(height.0, 2.0));
    BMI(bmi)
}
