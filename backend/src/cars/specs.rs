use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CarSpecification {
    Basic(String),
    Model(String),
}

impl CarSpecification {
    fn new_basic(value: &str) -> CarSpecification {
        CarSpecification::Basic(value.to_string())
    }
    fn new_models(value: &str) -> CarSpecification {
        CarSpecification::Model(value.to_string())
    }
}
