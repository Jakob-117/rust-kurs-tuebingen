use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
//pub struct Height(pub f64); //geht auch so, damit kann man mit Height.0 drauf zu greifen -> um .0 weg zu bekommen, muss man den Display trait implementieren (anstrengend)

pub struct Height {
    value: f64,
}

impl Height {
    pub fn new(height: f64) -> Height {
        //Ist eine funktion und braucht einen input
        Height { value: height }
    }

    pub fn value(&self) -> f64 {
        //ruft auf die instanz von Bmi auf
        self.value
    }
}
