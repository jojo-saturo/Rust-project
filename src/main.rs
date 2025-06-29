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

    println!("The number is {},with float {},Boolean {},The letter that start up my name {}.\n My friend {}.", num, dci, my_bool, alp, text);

    // Telling Rust the type data

    let fname: &str = "Josiah";
    let numb: i32 = 22;
    let height: f64 = 5.8;
    let ltr: char = 'J';
    let m_bool: bool = false;

    print!(" My name is {}\n I am {} years old\n I am {} tall\n My name start with letter {}\n You take pictures, {}.", fname, numb, height, ltr, m_bool);

    //Rust Constant
    // Note: when dealing with constant you have to incude the data type !!!
    const BIRTHYEAR: i32 = 2003;
    const FULL_NAME: &str = "Josiah Olaniyi";
    const PROG_LANG: &str = "Rust";

    println!(" I was born in the year {}", BIRTHYEAR);
    println!(" My full name is {}", FULL_NAME);
    println!(" I enjoy writing {}", PROG_LANG);
    
}
