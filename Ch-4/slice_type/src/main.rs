fn main() {
    demonstrate_string_slice();
    demonstrate_array_slice();
    demonstrate_slice_from_string();
    demonstrate_mutable_slice();
}

// Basic string slice example
fn demonstrate_string_slice() {
    println!("\n=== Basic String Slice ===");
    let str = String::from("Hello world");
    let result = first_word_len(&str);
    println!("First word length of '{}' is {}", str, result.len());
    println!("First word: '{}'", result);
}

// Array slice example
fn demonstrate_array_slice() {
    println!("\n=== Array Slice ===");
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4]; // Slice from index 1 to 3 (exclusive 4)
    println!("Full array: {:?}", numbers);
    println!("Slice [1..4]: {:?}", slice);
    println!("Slice length: {}", slice.len());
}

// Creating slices from String
fn demonstrate_slice_from_string() {
    println!("\n=== String Slicing Variations ===");
    let s = String::from("Rust Programming");

    let first = &s[0..4]; // "Rust"
    let middle = &s[5..11]; // "Progra"
    let end = &s[12..]; // "ming"
    let all = &s[..]; // Full string

    println!("Original: '{}'", s);
    println!("First part: '{}'", first);
    println!("Middle part: '{}'", middle);
    println!("End part: '{}'", end);
    println!("Full slice: '{}'", all);
}

// Working with mutable slices
fn demonstrate_mutable_slice() {
    println!("\n=== Mutable Slice ===");
    let mut numbers = [10, 20, 30, 40, 50];
    println!("Original array: {:?}", numbers);

    let slice = &mut numbers[1..4];
    slice[0] = 25; // Modify the slice (affects original array)

    println!("Modified slice: {:?}", slice);
    println!("Modified array: {:?}", numbers);
}

// Function to find first word length using string slice
fn first_word_len(input: &str) -> &str {
    let bytes: &[u8] = input.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // b' ' is byte literal for space
            return &input[..i];
        }
    }
    &input[..] // Return full string if no space found
}
