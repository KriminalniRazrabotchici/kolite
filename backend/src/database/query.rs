use std::collections::HashMap;

macro_rules! query {
    ({ $($key:ident: $value:expr),+ $(,)? }) => {{
        let mut query = Query::new();

        $(
            query.add(stringify!($key).to_string(), $value);
        )+

        query
    }};
}

pub struct Query <T>{
    body: HashMap<String, T>
}

// TODO implement a better way to store values, not only of one type
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

