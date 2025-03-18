fn main() {
    // Demonstrate different memory allocation types
    demonstrate_stack_allocation();
    demonstrate_heap_allocation();
    
    // Demonstrate ownership and borrowing concepts
    demonstrate_borrowing();
    demonstrate_ownership_transfer();
    demonstrate_ownership_return();
    demonstrate_full_ownership_cycle();
    
    // New demonstration: Ownership with tuple return
    demonstrate_length_calculation();
}

// Stack allocation example
fn demonstrate_stack_allocation() {
    println!("\n=== Stack Allocation ===");
    let x = 5;        // Simple integer stored on stack
    let y = x;        // Creates a copy (stack values are copied)
    println!("x: {}, y: {}", x, y);
}

// Heap allocation example with String
fn demonstrate_heap_allocation() {
    println!("\n=== Heap Allocation ===");
    let mut s = String::from("Hello");
    s.push_str(" World");
    s.push('!');
    println!("Modified string: {}", s);
}

// Borrowing demonstration
fn demonstrate_borrowing() {
    println!("\n=== Borrowing ===");
    let s1 = String::from("Hello");
    let s2 = &s1;
    println!("s1: {}, s2: {}", s1, s2);
}

// Ownership transfer to function
fn demonstrate_ownership_transfer() {
    println!("\n=== Ownership Transfer ===");
    let s3 = String::from("Hello");
    change_ownership(s3);
}

// Ownership received from function
fn demonstrate_ownership_return() {
    println!("\n=== Ownership Return ===");
    let s4 = give_ownership();
    println!("Received ownership: {}", s4);
}

// Full ownership cycle (take and return)
fn demonstrate_full_ownership_cycle() {
    println!("\n=== Ownership Take and Return ===");
    let s5 = String::from("Hello");
    let s6 = take_and_return_ownership(s5);
    println!("Returned ownership: {}", s6);
}

// New function: Calculate length while maintaining ownership
fn demonstrate_length_calculation() {
    println!("\n=== Ownership with Tuple Return ===");
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);  // s1 ownership moves to function
                                          // Returns tuple with string and length
    println!("The length of '{}' is {}.", s2, len);
    // s1 is no longer valid here as ownership was transferred
}

// Function that takes ownership
fn change_ownership(s: String) {
    println!("Ownership taken: {}", s);
}

// Function that gives ownership
fn give_ownership() -> String {
    let s = String::from("Give ownership");
    s
}

// Function that takes and returns ownership
fn take_and_return_ownership(s: String) -> String {
    println!("Ownership received: {}", s);
    s
}

// New function: Takes string and returns it with its length
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();  // Calculate the length of the string
    (s, length)           // Return tuple containing original string and its length
}