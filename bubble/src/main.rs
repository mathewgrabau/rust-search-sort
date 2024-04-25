use common;

fn main() {
    let result = common::get_i32("Hello, world - enter a number for the input: ");
    println!("you entered: {}", result);
}
