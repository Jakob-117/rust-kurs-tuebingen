use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
//pub struct Weight(pub f64); //8 Byte groß, dank Zero cost abstractions

pub struct Weight {
    value: f64,
}

impl Weight {
    pub fn new(weight: f64) -> Weight {
        //Ist eine funktion und braucht einen input
        Weight { value: weight }
    }

    pub fn value(&self) -> f64 {
        //ruft auf die instanz von Bmi auf
        self.value
    }
}
