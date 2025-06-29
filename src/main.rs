fn main() {
    println!("Hello, world!");
    println!("I am learning Rust.");
    println!("It is Intresting!");

    // Printing on the same line
    print!("Happy birthday to me ");
    print!("Executing on the same line \n");

    // Adding New line Manually (\n)
    print!("This the the first sentence \n");
    print!("Second sentence here. \n");

    // Broken line
    print!("Josiah\nLearning Rust \n");

    // Rust Variables
    // {} Placeholder In Rust

    let title = "Miss President";
    let name = "Eruobami Deborah Temiloluwa";
    let age = 21;

    println!("Her Excellency {}, {} is {} years old.", title, name, age);

    //Changing Variable value with mut
    let mut name = "Eruobami Deborah Temilola";
    println!("Before: {}", name);
    name = "Eruobami Deborah Temiloluwa";
    println!("After: {}", name);
    
    // Rust Data types
    let num = 9; // integer
    let dci = 3.56; // float
    let my_bool = true; // boolean
    let alp = 'J'; // character
    let text = "Kelechi"; // string

    println!("The number is {},with float {},Boolean {},The letter that start up my name {}.\n My frined {}.", num, dci, my_bool, alp, text);

}
