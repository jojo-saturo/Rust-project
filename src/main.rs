fn main() {
    /* println!("Hello, world!");
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

    println!(
        "The number is {},with float {},Boolean {},The letter that start up my name {}.\nMy friend {}.",
        num,
        dci,
        my_bool,
        alp,
        text
    );

    // Telling Rust the type data

    let fname: &str = "Josiah";
    let numb: i32 = 22;
    let height: f64 = 5.8;
    let ltr: char = 'J';
    let m_bool: bool = false;

    print!(
        "My name is {}\nI am {} years old\nI am {} tall\nMy name start with letter {}\nYou take pictures, {}.",
        fname,
        numb,
        height,
        ltr,
        m_bool
    );

    //Rust Constant
    // Note: when dealing with constant you have to incude the data type !!!
    const BIRTHYEAR: i32 = 2003;
    const FULL_NAME: &str = "Josiah Olaniyi";
    const PROG_LANG: &str = "Rust";

    println!("I was born in the year {}", BIRTHYEAR);
    println!("My full name is {}", FULL_NAME);
    println!("I enjoy writing {}", PROG_LANG);

    // Note: Adding data type for variables is optional unlike constant

    // Rust Operator
    let add = 20 + 9;
    let sub = 150 - 73;
    let mul = 65 * 14;
    let div = 400 / 20;
    let rem = 30 % 7;

    println!("Add {}", add);
    println!("Sub {}", sub);
    println!("Mul {}", mul);
    println!("Div {}", div);
    println!("Rem {}", rem);

    let mut x = 10;
    println!("Start {}", x);

    x += 5;
    println!("Addition {}", x);

    x -= 3;
    println!("Subtractio {}", x);

    x *= 2;
    println!("Multiplication {}", x);

    x /= 6;
    println!("Division {}", x);

    x %= 9;
    println!("After {}", x);

    // Comparision Operator (true/false)
    let a = 13;
    let y = 7;

    println!("13 == 7: {}", a == y);
    println!("13 != 7: {}", a != y);

    //  If...else Statement
    let age = 16;

    if age >= 18 {
        println!("You can vote!");
    } else {
        println!("You cannot vote yet!");
    }

    // else if Statement
    let score = 79;

    if score >= 70 {
        println!("Distinction: A");
    } else if score >= 60 {
        println!("Excellent: B");
    } else if score >= 50 {
        println!("Good: C");
    } else if score >= 40 {
        println!("Pass: D");
    } else {
        println!("Fail: F");
    }

    // Using if as an expression
    let num = 10;

    let result = if num % 2 == 0 { "Even" } else { "Odd" };
    println!("The number {} is {}", num, result);

    let time = 15; // 3 PM in 24-hour format

    let greeting = if time < 12 {
        "Good morning!"
    } else if time < 18 {
        "Good afternoon!"
    } else {
        "Good evening!"
    };

    println!("{}", greeting);

    // Rust Match
    let day = 4;

    match day {
        1 => println!("Sunday"),
        2 => println!("Monday"),
        3 => println!("Tuesday"),
        4 => println!("Wednesday"),
        5 => println!("Thursday"),
        6 => println!("Friday"),
        7 => println!("Saturday"),
        _ => println!("Invalid day."),
    }

    let day = 6;

    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day."),
    }

    // Returning values from match

    let day = 3;

    let result = match day {
        1 => "Sunday",
        2 => "Monday",
        3 => "Tuesday",
        4 => "Wednesday",
        5 => "Thursday",
        6 => "Friday",
        7 => "Saturday",
        _ => "Invalid day.",
    };
    println!("{}", result);

    // Loop in Rust

    loop {
        println!("Olaniyi Josiah");
        break; // Exit the loop after one iteration
    }

    let mut count = 1;

    loop {
        println!("Welcome to Rust!");

        if count == 3 {
            break; // Exit the loop when count reaches 1
        }

        count += 1; // Decrement count
    }

    let mut count = 1;

    let result = loop {
        println!("Goodbye to TypeScript!");
        if count == 6 {
            break count;
        }
        count += 1;
    };
    println!("Count is: {}", count); */

    // Compund Data Types
    comp_data_types();
}

fn comp_data_types() {
    // Array, Turples, slices and String(String slice)

    // Array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    //println!("Numbers of Array: {}", numbers); Display format for arrays
    // We cant use {} here because it only works with string, we can use {:?} to print array
    println!("Numbers of Array: {:?}", numbers); // Debug format for arrays

    // Debugable format print all the elements in the array

    // Display format print only one element in the array

    // Calling an array element
    println!("Third element: {}", numbers[2]);
    println!("Last element: {}", numbers[4]);

    let name: [&str; 3] = ["Eruobami", "Deborah", "Temiloluwa"];
    println!("Names Array: {:?}", name);

    println!("Last Name: {}", name[2]);
    println!("First Name: {}", name[0]);
    println!("Second Name: {}", name[1]);

    // Tuples
    let human: (String, i32, f64) = ("Josiah".to_string(), 22, 165.3);

    print!(
        "Human Tuple: {:?}\nName:{}\nAge:{} years old\nHeight:{}cm \n",
        human,
        human.0,
        human.1,
        human.2
    );
    // tuples use parenthesis () and can hold different data types
    // Accessing tuple elements using index var.0, var.1, var.2, etc.

    //let mix_turple: (i32, String, bool) = (49, "Apple".to_string(), true);
    let mix_turple = (49, "Apple", true);

    print!(
        "Mix_turple: {:?} \n{} is a perfect square \nI love {} \nIs second element a fruit? {}",
        mix_turple,
        mix_turple.0,
        mix_turple.1,
        mix_turple.2
    );
}
