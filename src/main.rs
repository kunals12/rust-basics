fn main() {

    // Numbers
    let x: i8 = -2;
    let y: u8 = 2;
    let z: f32 = 2.3;

    println!("x : {}, y: {},z: {}", x, y,z);

    // Boolean
    let is_male: bool = false;
    let is_above_18: bool = true;
    
    if is_male {
        println!("You are a male");

    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        println!("You are a legal male");
    }

    // Strings
    let greeting = String::from("Welcome To Rust");
    println!("{}", greeting);

    let char1 = greeting.chars().nth(1);

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character at index"),
    }
}
