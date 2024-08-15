fn main() {
    // define a Person struct
    struct Person {
        name: String,
        age: u8,
        height: u8
    }
    
    // instantiate Person struct
    let person = Person {
        name: String::from("John Doe"),
        age: 18,
        height: 178
    };
    
    println!("Person name = {}", person.name);
    println!("Person age = {} years", person.age);
    println!("Person height = {} cm", person.height);

    // destructure Person struct into name, age and height variables
    let Person { name, age, height } = person;
    println!("After Destruct");
    println!("Person name = {}", name);
    println!("Person age = {} years", age);
    println!("Person height = {} cm", height);
}