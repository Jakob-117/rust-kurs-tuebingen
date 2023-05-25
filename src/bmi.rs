#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Bmi {
    value: f64,
}

impl Bmi {
    pub fn new(bmi: f64) -> Bmi {
        //Ist eine funktion und braucht einen input
        Bmi { value: bmi }
    }

    pub fn value(&self) -> f64 {
        //ruft auf die instanz von Bmi auf
        self.value
    }
}
