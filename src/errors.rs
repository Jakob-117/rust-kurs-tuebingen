#[derive(Debug, PartialEq)]
pub enum BmiError {
    HeightIsZero,
    HeightIsNegative,
    WeightIsNotOk,
}

#[derive(Debug)]
pub enum InputError {
    //InputIsntStd
}
