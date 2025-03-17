fn main() {
    // declare variable using let.
    let x = 20; // immutable chant change value in future.
    println!(" x value is {}", x);

    // change value using mut key word.But can't change datatype.
    let mut y = 30; // mutable,
    println!("y is changed {} ", y);
    y = 40;
    println!("y is changed {} ", y);

    // use shadowing. you can change datatype and value of let variable.
    let user = "xyz";
    println!("{user}"); // xyz
    let user = true;
    println!("{user}"); // true

    // const
    const NAME: &str = "Hello";
    let new_name = format!("{} Sir", NAME); // Works with constants
    println!(" {} form RUST ", new_name);

    const AGE: u32 = 20;
    let new_age = AGE + 5;
    println!(" {} ", new_age);
}
