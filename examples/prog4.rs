fn main() {
    // assign a floating point f64 value to decimal variable
    let decimal: f32 = 64.93;

    // convert decimal variable to u16 integer type using as keyword
    let integer = decimal as u32;

    println!("decimal = {}", decimal);
    println!("integer = {}", integer);
}