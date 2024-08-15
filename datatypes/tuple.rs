fn main() {
    let mixture = ("Hello, World!", 16, 2.71828);
    println!("{}",mixture.0);
    // destructuring a tuple
    let (message, number, float) = mixture;
    
    println!("message = {}", message);
    println!("number = {}", number);
    println!("float = {}", float);
}