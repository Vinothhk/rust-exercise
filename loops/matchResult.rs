fn main() {
    let my_result: Result<i32, &str> = Ok(200);

    // use of match expression to match Result type
    match my_result {
        Ok(value) => println!("The result is {}", value),
        Err(error) => println!("The error message is {}", error),
    }
}