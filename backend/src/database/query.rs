use std::collections::HashMap;

pub struct Query <T>{
    body: HashMap<String, T>
}

impl <T> Query<T> {
    pub fn new() -> Self {
        Query {
            body: HashMap::new()
        }
    }

    pub fn add(&mut self, key: String, value: T) {
        self.body.insert(key, value);
    }
}

pub trait AbleToQuery<T> {
    fn get_as_query(&self) -> Query<T>;
}
