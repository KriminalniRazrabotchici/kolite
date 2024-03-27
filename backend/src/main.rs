mod users;
mod db_operations;
mod errors;


// TODO complete a registration
fn main() {
    let mut data = vec![1, 2, 3];
    let mut x = &data[0];

    println!("{}", x);
    data.push(3); 
    x = &data[0];
    println!("{}", x);

    println!("Hello, world!");
}
