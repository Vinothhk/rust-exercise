fn main() {
    let mut number = 0;
    
    // infinite loop starts here
    loop {
        number += 1;
        println!("{}", number);
        
        if number >= 10 {
            // exit the loop
            break;
        }
    }
}