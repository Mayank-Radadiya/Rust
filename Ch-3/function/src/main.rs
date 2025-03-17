// https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html
fn main() {
    println!("Hello, world!");

    let x = 10;
    let y = 20;

    let total = sum(x, y);
    println!("The sum of {x} and {y} is: {}", total);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
