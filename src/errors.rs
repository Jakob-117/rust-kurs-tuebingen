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

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    IntermediateOffset(#[from] time::error::IndeterminateOffset),
}
