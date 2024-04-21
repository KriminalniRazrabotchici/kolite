use serde::{Deserialize, ser::{Serializer, Serialize, SerializeStruct}};

#[derive(Deserialize)]
pub struct CarSpecificationResponse {
    spec_type: String,
    values: Vec<String>
}

impl Serialize for CarSpecificationResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer 
    {
        let mut state = serializer.serialize_struct("CarSpecificationResponse", 1)?;
 
        let value = self.spec_type.to_owned();
        let value: &'static str = Box::leak(value.into_boxed_str());
        state.serialize_field(value, &self.values)?;
        state.end()
    }
}
