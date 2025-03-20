// Regular struct definition
struct User {
    username: String,
    age: u32,
    active: bool,
}

// Tuple struct definition
struct Point(i32, i32, i32);

fn main() {
    demonstrate_struct_creation();
    demonstrate_field_modification();
    demonstrate_ownership_transfer();
    demonstrate_struct_update_syntax();
    demonstrate_tuple_struct();
}

// Basic struct creation and access
fn demonstrate_struct_creation() {
    println!("\n=== Struct Creation ===");
    let user1 = User {
        username: String::from("xyz"),
        age: 20,
        active: true,
    };

    println!("Initial username: {}", user1.username);
    println!("Age: {}, Active: {}", user1.age, user1.active);
}

// Modifying struct fields and ownership
fn demonstrate_field_modification() {
    println!("\n=== Field Modification ===");
    let mut user1 = User {
        username: String::from("xyz"),
        age: 20,
        active: true,
    };

    // Modify field
    user1.username = String::from("new name");
    user1.username.push_str(" push new string");
    println!("Modified username: {}", user1.username);
}

// Ownership transfer from struct field
fn demonstrate_ownership_transfer() {
    println!("\n=== Ownership Transfer ===");
    let mut user1 = User {
        username: String::from("original"),
        age: 20,
        active: true,
    };

    let s1 = user1.username; // Ownership of username moves to s1
    println!("Transferred username: {}", s1);
    // println!("{}", user1.username);  // This would fail

    // Can assign new value as field is still there
    user1.username = String::from("new username");
    println!("New username assigned: {}", user1.username);
}

// Struct update syntax and instance creation
fn demonstrate_struct_update_syntax() {
    println!("\n=== Struct Update Syntax ===");
    let s2 = String::from("hello user");
    let user2 = create_instances_of_structs(s2, 30, false);
    println!(
        "User2 - username: {}, age: {}, active: {}",
        user2.username, user2.age, user2.active
    );

    let user3 = User {
        username: String::from("user-3"),
        ..user2 // Copy remaining fields from user2
    };
    println!(
        "User3 - username: {}, age: {}, active: {}",
        user3.username, user3.age, user3.active
    );

    // user2.username is moved to user3, but stack data (age, active) is copied
    // println!("{}", user2.username);  // This would fail
    println!(
        "User2 remaining fields - age: {}, active: {}",
        user2.age, user2.active
    );
}

// Tuple struct demonstration
fn demonstrate_tuple_struct() {
    println!("\n=== Tuple Struct ===");
    let point = Point(10, 20, 30);

    // Access fields using dot notation with index
    println!(
        "Point coordinates: x={}, y={}, z={}",
        point.0, point.1, point.2
    );

    // Destructuring tuple struct
    let Point(x, y, z) = point;
    println!("Destructured: x={}, y={}, z={}", x, y, z);
}

// Function to create struct instance
fn create_instances_of_structs(username: String, age: u32, active: bool) -> User {
    User {
        username,
        age,
        active,
    }
}
