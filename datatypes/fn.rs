// define an add function that takes in two parameters with a return type
fn add(a: i32, b: i32) -> i32 {
    let sum = a + b;

    // return a value from the function
    return sum;
}

fn main() {
    // function call
    let sum = add(3, 5);

    println!("Sum of a and b = {}", sum);
}