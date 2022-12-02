pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    println!("Number: {}", 1);

    println!("My name is {} and I live in {}", "Fred", "England");

    println!("I am {0} and {0} likes to code in {1}.", "Mark", "Rust");

    println!("{name} likes to play {activity}",
        name = "Dave",
        activity = "Football"
    );

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal {:o}" , 10, 10, 10);

    // Placeholder for Debug Trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
