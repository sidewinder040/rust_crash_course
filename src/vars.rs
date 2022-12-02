pub fn run() {
    let name = "Mark";
    let mut age = 57;
    println!("My name is {}, and I am {} years old.", name, age);
    age = 58;
    println!("It's my Birthday, I'm now {}", age);
    // Define Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple Vars
    let (name2, age2) = ("Mary", 68);
    println!("{} is {}", name2, age2);
}