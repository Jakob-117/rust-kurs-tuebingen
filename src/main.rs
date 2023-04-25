use std::io::Stdin;
use std::str::FromStr;
struct Weight(f64); //8 Byte groß, dank Zero cost abstractions

struct Height(f64); //geht auch so, damit kann man mit Height.0 drauf zu greifen -> um .0 weg zu bekommen, muss man den Display trait implementieren (anstrengend)

struct Bmi(f64);
/*
impl Drop for BMI{ //destructor for structs -> normally generated from the compiler -> braucht man praktisch nie
    fn drop(&mut self){ //kein quatsch in diesem Block machen, man kann mit &mut noch innerhalb was ändern, aber wird dann sofort gedroppt
        todo!()
    }
}*/

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

    println!("Dein BMI: {}", bmi.0);
}

fn get_input(stdin: &Stdin) -> f64 {
    let mut buffer_height = String::new();
    match stdin.read_line(&mut buffer_height) {
        Ok(_usize) => f64::from_str(buffer_height.trim()).unwrap_or_else(|err| {
            println!("There was a parsing error: {}", err);
            println!("Try again!");
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
fn calculate_bmi(height: Height, weight: Weight) -> Bmi {
    if height.0 == 0.0 {
        println!("Cannot divide by zero!");
        println!("Try again!");
        start_bmi_calculation()
    }
    let bmi = weight.0 / (f64::powf(height.0, 2.0));
    Bmi(bmi)
}

/*
#[cfg(test)]
mod tests{
    use super::get_input;
    #[test]
    fn test_input(){
        let mut stdin = std::io::stdin();

    }
}
*/
