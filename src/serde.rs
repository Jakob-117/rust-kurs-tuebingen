use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::height::Height;
use crate::weight::Weight;

#[derive(Serialize, Deserialize)]
pub struct BmiJson {
    pub(crate) height: Height,
    pub(crate) weight: Weight,
    pub(crate) time: Duration,
}
