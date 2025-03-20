// Derive Debug trait for printing struct
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

// Implementation block for Rectangle
impl Rectangle {
    // Method to calculate area (instance method)
    fn calculate_area(&self) -> u32 {
        self.height * self.width
    }

    // Method to check if this rectangle can hold another
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function (static method) to create a square
    fn square(side: u32) -> Self {
        Self {
            height: side,
            width: side,
        }
    }
}

fn main() {
    demonstrate_basic_usage();
    demonstrate_comparison();
    demonstrate_square_creation();
}

// Basic struct usage and area calculation
fn demonstrate_basic_usage() {
    println!("\n=== Basic Struct Usage ===");
    let rect = Rectangle {
        width: 10,
        height: 15,
    };

    println!("Rectangle: {:?}", rect);
    println!("Area of rectangle is {}", rect.calculate_area());
}

// Comparing rectangles
fn demonstrate_comparison() {
    println!("\n=== Rectangle Comparison ===");
    let rect1 = Rectangle {
        width: 10,
        height: 15,
    };

    let rect2 = Rectangle {
        width: 7,
        height: 10,
    };

    let can_hold = rect1.can_hold(&rect2);
    println!("Rect1: {:?}", rect1);
    println!("Rect2: {:?}", rect2);
    println!("Can rect1 hold rect2: {}", can_hold);
}

// Using associated function to create square
fn demonstrate_square_creation() {
    println!("\n=== Square Creation ===");
    let square = Rectangle::square(10);
    println!("Square: {:?}", square);
    println!("Square area: {}", square.calculate_area());
}
