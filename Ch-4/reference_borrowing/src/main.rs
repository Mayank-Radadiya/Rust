fn main() {
    demonstrate_immutable_borrowing();
    demonstrate_mutable_borrowing();
    demonstrate_multiple_immutable_borrows();
    demonstrate_borrowing_rules();
    demonstrate_scope_resolution();
    demonstrate_practical_example();
}

// Basic immutable borrowing
fn demonstrate_immutable_borrowing() {
    println!("\n=== Immutable Borrowing ===");
    let message = String::from("Hello, world!");
    let length = string_length(&message); // Immutable borrow
    println!("The length of '{}' is {}", message, length);
    // Original 'message' remains valid as we only borrowed it
}

// Basic mutable borrowing
fn demonstrate_mutable_borrowing() {
    println!("\n=== Mutable Borrowing ===");
    let mut text = String::from("My Name ");
    change_string(&mut text); // Mutable borrow to modify the string
    println!("Modified string: '{}'", text);
}

// Multiple immutable borrows
fn demonstrate_multiple_immutable_borrows() {
    println!("\n=== Multiple Immutable Borrows ===");
    let greeting = String::from("hello");
    let ref1 = &greeting; // First immutable borrow
    let ref2 = &greeting; // Second immutable borrow
    println!("Multiple immutable refs: '{}' and '{}'", ref1, ref2);
    // Multiple immutable borrows are allowed
}

// Borrowing rules demonstration
fn demonstrate_borrowing_rules() {
    println!("\n=== Borrowing Rules ===");
    let mut data = String::from("hello");

    // This would cause an error - can't have immutable borrow after mutable
    // let r1 = &mut data;
    // let r2 = &data;  // Cannot borrow as immutable after mutable

    let ref1 = &data; // First immutable borrow
    let ref2 = &data; // Second immutable borrow
    println!("Two immutable refs: '{}' and '{}'", ref1, ref2);

    // Can't create mutable borrow while immutable borrows exist
    // let ref3 = &mut data;  // This would fail

    // But we can create a mutable borrow after the immutable ones over.
    let mut my_name = String::from("My name is ");

    let new_ref = &my_name; // Immutable borrow
    let new_ref2 = &my_name; // Another immutable borrow
    println!("Immutable refs: '{}' and '{}'", new_ref, new_ref2);

    // now i can create a mutable borrow
    let mut_ref = &mut my_name; // Mutable borrow
    mut_ref.push_str("John Doe"); // Modify through mutable reference
    println!("Modified mutable ref: '{}'", mut_ref);
}

// Scope-based borrowing resolution
fn demonstrate_scope_resolution() {
    println!("\n=== Scope-Based Borrowing ===");
    let mut phrase = String::from("my name is");
    println!("Original: '{}'", phrase);

    {
        let ref1 = &mut phrase; // Mutable borrow in inner scope
        ref1.push_str(" John"); // Modify through mutable reference
        println!("Modified in scope: '{}'", ref1);
    } // ref1 goes out of scope here

    println!("After scope: '{}'", phrase);
    let ref2 = &mut phrase; // New mutable borrow is allowed
    ref2.push_str(" Doe");
    println!("After second modification: '{}'", ref2);
}

// Practical example combining concepts
fn demonstrate_practical_example() {
    println!("\n=== Practical Example ===");
    let mut document = String::from("Report: ");

    // Get length with immutable borrow
    let doc_length = string_length(&document);
    println!("Initial length: {}", doc_length);

    // Modify with mutable borrow
    {
        let doc_ref = &mut document;
        append_content(doc_ref);
    }

    // Check length again
    let new_length = string_length(&document);
    println!("Updated document: '{}'", document);
    println!("New length: {}", new_length);
}

// Function using immutable reference
fn string_length(s: &String) -> usize {
    s.len() // No need for explicit return
}

// Function using mutable reference
fn change_string(s: &mut String) {
    s.push_str("Xyz");
    println!("Inside function: '{}'", s);
}

// Additional helper function for practical example
fn append_content(s: &mut String) {
    s.push_str("Sales figures for Q1");
}
