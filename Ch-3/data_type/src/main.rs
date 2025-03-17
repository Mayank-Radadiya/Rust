fn main() {
    // INTEGER TYPES DEMONSTRATION
    demonstrate_integer_types();

    // INTEGER OVERFLOW HANDLING
    demonstrate_integer_overflow();

    // FLOATING POINT TYPES
    demonstrate_floating_points();

    // BOOLEAN TYPES
    demonstrate_booleans();

    // CHARACTER TYPES
    demonstrate_characters();

    // NUMERIC OPERATIONS
    demonstrate_numeric_operations();

    // COMPOUND DATA TYPE
    compound_data_type();
}

// Integer Types Section
fn demonstrate_integer_types() {
    println!("\n=== Integer Types ===");

    // Signed and Unsigned Integers
    let signed: i32 = 10; // Can store positive and negative numbers
    let unsigned: u32 = 50; // Only stores positive numbers
    let negative: i32 = -20; // Signed with negative value
    println!(
        "Signed: {}, Unsigned: {}, Negative: {}",
        signed, unsigned, negative
    );

    // Alternative Integer Notation
    let small_unsigned = 1u8; // Suffix notation
    let small_signed = 20_i8; // Suffix with underscore
    let large_number = 10_00_00; // Visual separator with underscores
    println!(
        "Small unsigned: {}, Small signed: {}, Large number: {}",
        small_unsigned, small_signed, large_number
    );

    // Different Number Systems
    let decimal = 98_222; // Decimal notation
    let hex = 0xff; // Hexadecimal (255)
    let octal = 0o77; // Octal (63)
    let binary = 0b1111_0000; // Binary (240)
    let byte = b'A'; // Byte value of ASCII 'A' (65)
    println!(
        "Decimal: {}, Hex: {}, Octal: {}, Binary: {}, Byte: {}",
        decimal, hex, octal, binary, byte
    );
}

// Integer Overflow Handling Section
fn demonstrate_integer_overflow() {
    println!("\n=== Integer Overflow Handling ===");

    // 1. Wrapping Method
    let wrap_num: u8 = 255;
    let wrapped = wrap_num.wrapping_add(1); // Wraps around to 0
    println!("Wrapping method: 255 + 1 = {}", wrapped);

    // 2. Overflowing Method
    let overflow_num: u8 = 255;
    let (result, overflow) = overflow_num.overflowing_add(1);
    println!(
        "Overflowing method: 255 + 1 = {}, Overflowed: {}",
        result, overflow
    );

    // 3. Saturating Method
    let sat_num: u8 = 255;
    let saturated = sat_num.saturating_add(1); // Stays at maximum value
    println!("Saturating method: 255 + 1 = {}", saturated);

    // 4. Checked Method
    let check_num: u8 = 255;
    let checked = check_num.checked_add(1);
    match checked {
        Some(n) => println!("Checked method: 255 + 1 = {}", n),
        None => println!("Checked method: 255 + 1 = None (overflow occurred)"),
    }
}

// Floating Point Types Section
fn demonstrate_floating_points() {
    println!("\n=== Floating Point Types ===");
    let float32: f32 = 2.0; // 32-bit floating point
    let float64: f64 = 3.0; // 64-bit floating point
    println!("32-bit float: {}, 64-bit float: {}", float32, float64);
}

// Boolean Types Section
fn demonstrate_booleans() {
    println!("\n=== Boolean Types ===");
    let truth = true;
    let falsehood: bool = false;
    println!("True: {}, False: {}", truth, falsehood);
}

// Character Types Section
fn demonstrate_characters() {
    println!("\n=== Character Types ===");
    let lowercase: char = 'z';
    let uppercase: char = 'Z';
    let emoji: char = '😻';
    println!(
        "Lowercase: {}, Uppercase: {}, Emoji: {}",
        lowercase, uppercase, emoji
    );
}

// Numeric Operations Section
fn demonstrate_numeric_operations() {
    println!("\n=== Numeric Operations ===");

    let sum = 5 + 10; // Addition
    let difference = 95.5 - 4.3; // Subtraction
    let product = 4 * 30; // Multiplication
    let quotient = 56.7 / 32.2; // Division
    let truncated = -5 / 3; // Integer division
    let remainder = 43 % 5; // Modulus

    println!(
        "Sum: {}, Difference: {}, Product: {}, Quotient: {}, Truncated: {}, Remainder: {}",
        sum, difference, product, quotient, truncated, remainder
    );
}

fn compound_data_type() {
    // tuple
    println!("\n=== Compound Data Type: Tuple ===");

    let tuple: (i32, f64, char) = (500, 6.4, 'z');
    let (x, y, z) = tuple; // Destructuring

    println!("Tuple: x = {}, y = {}, z = {}", x, y, z);
    println!("Tuple: x = {0}, y = {1}, z = {2}", x, y, z); // Positional arguments
    println!("Tuple first element: {}", tuple.0); // Accessing tuple elements
    println!("Tuple: {:?}", tuple); // Debug print  // => (500, 6.4, 'z')
    println!("Tuple: {:#?}", tuple); // Pretty print
                                     // => (
                                     //     500,
                                     //     6.4,
                                     //     'z',
                                     // )

    // Array
    println!("\n=== Compound Data Type: Array ===");
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // Fixed-size array. Can't change size
    let array2 = [1, 2, 3, 4, 5]; // Type inference
    let array3 = [3; 5]; // Array with same value
    println!("Array: {:?}", array); // Debug print
    println!("Array: {:#?}", array); // Pretty print
                                       // => [
                                       //     1,
                                       //     2,
                                       //     3,
                                       //     4,
                                       //     5,
                                       // ]
    println!("Array2: {:?}", array2); // Debug print
    println!("Array3: {:?}", array3); // Debug print
    println!("Array2 first element: {}", array2[0]); // Accessing array elements
    println!("Array2 length: {}", array2.len()); // Length of the array
    println!("Array2 size in bytes: {}", std::mem::size_of_val(&array2)); // Size in bytes
    println!("Array2 first element: {}", array2.first().unwrap()); // First element
    println!("Array2 last element: {}", array2.last().unwrap()); // Last element
    println!("Array2 slice: {:?}", &array2[1..3]); // Slice of the array
    println!("Array2 slice: {:?}", &array2[..3]); // Slice of the array
    println!("Array2 slice: {:?}", &array2[1..]); // Slice of the array
    println!("Array2 slice: {:?}", &array2[..]); // Slice of the array
    println!("Array2 slice: {:?}", &array2[1..=3]); // Slice of the array
}

