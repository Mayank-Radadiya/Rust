// Derive Debug for pretty-printing struct instances
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    demonstrate_basic_area_calculation();
    demonstrate_multiple_rectangles();
    demonstrate_validation();
}

// Basic area calculation with struct
fn demonstrate_basic_area_calculation() {
    println!("\n=== Basic Area Calculation ===");
    let rect = Rectangle {
        height: 5,
        width: 6,
    };

    let area = count_area(&rect); // Pass reference to avoid ownership transfer
    println!("Rectangle: {:#?}", rect);
    println!("Area: {}", area);
    // rect remains valid here because we only borrowed it
}

// Working with multiple rectangles
fn demonstrate_multiple_rectangles() {
    println!("\n=== Multiple Rectangles ===");
    let rect1 = Rectangle {
        height: 10,
        width: 20,
    };

    let rect2 = Rectangle {
        height: 3,
        width: 4,
    };

    println!("Rectangle 1: {:#?}", rect1);
    println!("Area 1: {}", count_area(&rect1));
    println!("Rectangle 2: {:#?}", rect2);
    println!("Area 2: {}", count_area(&rect2));
}

// Adding basic validation
fn demonstrate_validation() {
    println!("\n=== Rectangle with Validation ===");

    // Create rectangle with potentially invalid values
    let invalid_rect = Rectangle {
        height: 0,
        width: 5,
    };

    match validated_area(&invalid_rect) {
        Ok(area) => println!("Valid rectangle {:#?}\nArea: {}", invalid_rect, area),
        Err(e) => println!("Invalid rectangle {:#?}\nError: {}", invalid_rect, e),
    }

    let valid_rect = Rectangle {
        height: 8,
        width: 10,
    };

    match validated_area(&valid_rect) {
        Ok(area) => println!("Valid rectangle {:#?}\nArea: {}", valid_rect, area),
        Err(e) => println!("Invalid rectangle {:#?}\nError: {}", valid_rect, e),
    }
}

// Basic area calculation function using reference
fn count_area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

// Area calculation with validation
fn validated_area(rect: &Rectangle) -> Result<u32, String> {
    if rect.height == 0 || rect.width == 0 {
        Err(String::from("Height and width must be greater than 0"))
    } else {
        Ok(rect.height * rect.width)
    }
}
