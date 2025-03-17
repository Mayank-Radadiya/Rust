fn main() {
    demonstrate_conditionals();
    demonstrate_conditional_expression();
    demonstrate_nested_conditionals();
    demonstrate_loops();
}

fn demonstrate_conditionals() {
    println!("\n=== Basic If-Else ===");
    let x = 2;
    
    // Simple if-else for even/odd check
    if x % 2 == 0 {
        println!("Even");  // Fixed the logic (was incorrectly printing "Odd")
    } else {
        println!("Odd");
    }
    
    // Note: Rust doesn't support falsy/truthy values like some languages
    // This won't work:
    // if x { ... }
}

fn demonstrate_conditional_expression() {
    println!("\n=== If as Expression ===");
    // If can be used as an expression to assign values
    let num = if 5 > 3 { 5 } else { 3 };
    println!("The greater number is {}", num);
}

fn demonstrate_nested_conditionals() {
    println!("\n=== Nested If Statements ===");
    let age = 20;
    
    if age >= 18 {
        println!("Adult");
        if age >= 65 {
            println!("Senior");
        } else {
            println!("Not Senior");
        }
    } else {
        println!("Minor");
    }
}

fn demonstrate_loops() {
    println!("\n=== Loop Examples ===");
    demonstrate_infinite_loop();
    println!("\n=== While Loop ===");
    demonstrate_while_loop();
    println!("\n=== For Loop ===");
    demonstrate_for_loop();
}

fn demonstrate_infinite_loop() {
    // Basic infinite loop with break
    let mut count = 0;
    println!("Basic loop counting to 5:");
    loop {
        count += 1;
        print!("{} ", count);
        if count == 5 {
            break;
        }
    }
    println!();  // New line after the sequence

    // Loop as an expression returning a value
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 5 {
            break count * 2;
        }
    };
    println!("Loop returning value: {}", result);

    // Loop with labels for nested control
    'outer: loop {
        println!("Outer loop");
        loop {
            println!("Inner loop");
            break 'outer;  // Breaks out of both loops
        }
    }
}

fn demonstrate_while_loop() {
    let mut count = 0;
    while count < 5 {
        count += 1;
        print!("{} ", count);
    }
    println!();  // New line after the sequence
}

fn demonstrate_for_loop() {
    // For loop with array
    let numbers = [10, 20, 30, 40, 50];
    println!("Iterating over array:");
    for number in numbers {
        print!("{} ", number);
    }
    println!();

    // For loop with inclusive range
    println!("Range (100 to 111 inclusive):");
    for x in 100..=111 {
        print!("{} ", x);
    }
    println!();

    // For loop with reverse range and step
    println!("Reverse range with step of 2:");
    for x in (1..=10).rev().step_by(2) {
        print!("{} ", x);
    }
    println!();
}