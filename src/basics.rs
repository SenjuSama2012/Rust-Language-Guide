#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]

pub fn sec2_basics() {
    scalar_data_types();
    tuples();
    arrays();
    vectors();
    slices();
    strings_str();
    string_literals();
    functions();
    control_flow();
}

fn scalar_data_types() {
    //Variables Mutability and Scalar Data Types
    let x: i8 = 10;
    println!("{}", x);

    let _y: u8 = 10;

    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("{}", decimal);
    println!("{}", hex);
    println!("{}", octal);
    println!("{}", binary);

    let byte = b'A';
    println!("{}", byte);

    let x = 2.0; //f64 default
    let y: f32 = 1.0;

    let t = true;
    let f: bool = false;

    let c = 'c';

    println!("{}", c);

    //Operators
    // + - * / %

    let a = 10;
    let b = 4;

    let remainder = a % b;
    println!("{}", remainder);
}

fn tuples() {
    //  Tuples
    //      -contain different data types.
    //      -Tuples have a fixed length so they cannot grow or shrink in size.

    let tup = (500, "Rust", true);
    println!("{}", tup.2);

    let tup = (500, "hi", true);
    println!("{}", tup.2);
    let (x, y, z) = tup;

    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
}

fn arrays() {
    //Arrays
    //  -Must contain the same type
    //  -Have a fixed length so they cannot grow or shrink in size.

    let array = [1, 2, 3];

    println!("{}", array[0]);

    let mut array2: [i32; 3] = [4, 5, 6];
    println!("{}", array2[0]);
    array2[0] = 10;
    println!("{}", array2[2]);
}

fn vectors() {
    //  Vectors
    //      -Vectors are a sizeable array of elements allocated on the HEAP.
    //      -Vectors can grow and shrink.
    //      -Vectors are VERY Commonly used in Rust.

    let mut nums = vec![1, 2, 3];

    nums.push(4);
    println!("{:?}", nums);
    nums.pop();
    println!("{:?}", nums);

    let mut vec = Vec::new(); //vec!
    vec.push("Test");
    vec.push("String");
    println!("{:?}", vec);

    vec.reverse();
    println!("{:?}", vec);

    let mut vect = Vec::<i32>::with_capacity(2);
    println!("{}", vect.capacity());

    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    let sv: &[i32] = &v[2..4];
    println!("{:?}", sv);
}
fn slices() {
    // Strings
    //  -Are stored as a vector of bytes
    //  -Are stored on the Heap
    let name = String::from("Tyler");
    let course = "Rust".to_string();
    let new_name = name.replace("Tyler", "Ty");

    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);
}
fn strings_str() {
    // &str = "string slice" or "stir"
    // String slice DOES NOT allocate memory on the Heap.
    // String Slice is a fixed length.
    let str1 = "hello"; //&str
    let str2 = str1.to_string();
    let str3 = &str2;

    println!("{}", str1);
    println!("{}", str2);
    println!("{}", str3);

    // Compare string == != (does not equal)
    println!("{}", "ONE".to_lowercase() == "one");
}
fn string_literals() {
    let rust = "\x52\x75\x73\x74";
    println!("{}", rust);
}
fn functions() {
    // Functions
    //      -Functions use: snake-case naming convention
    //      -(): These are where arguments and placed inside of the function syntax;
    //      -You can pass arguments to the functions
    //          -Add Argument to Function Call
    //          -Define argument and data type within the functions argument
    //              Example: THE OUTPUT OF THE variable (phrase) BELOW
    //      -
    //      - You can Return Values.
    //          - To return values in a function, you end WITHOUT ";"
    //      -
    //      -
    //      -

    print_phase("Print my Argument");
    println!("{}", gcd(20, 5));
    println!("{}", multiple_return_values(true));

    fn print_phase(phrase: &str) {
        println!("Whaddup Doe Rustaceans!");
        println!("{}", phrase)
    }

    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while a != 0 {
            if a < b {
                let c = a;
                a = b;
                b = c;
            }
            a = a % b;
        }
        b
    }

    fn multiple_return_values(flag: bool) -> bool {
        if flag == true {
            true
        } else {
            false
        }
    }
}

fn control_flow() {
    //      Control Flow
    //          -Control Flow is deciding where to run code based on a "If" condition is true or not
    //          -OR deciding if to repeadtedly continue running some code while the condition is true.
    //          -
    //          -Rust uses 3 Common Types of Control Flow Statments
    //              -If Statements
    //                  -Else If Statements
    //              -Loop
    //              -While Loops
    //              -For Loops
    //          -
    //          -
    //          -
    //

    if_statements();
    // simple_loop(); //This is an Inifinite Loop
    counter();
    while_loops();
    for_loop();
    for_loop_2();

    fn if_statements() {
        let one = 1;

        if one > 0 {
            println!("True");
        } else if one == 1 {
            println!("Equal");
        } else {
            println!("False");
        }
    }

    fn simple_loop() {
        loop {
            println!("Loop");
        }
    }

    // PSA: I didn't know you could name Loops in Rust, This concept is new and a lil jarring.
    //          Review this a couple times to understand how this was done.
    fn counter() {
        println!("\n\nThis is a concept I didn't know about for loops. Review how \"Naming loops\" can be used.");
        let mut num = 0;

        'counter: loop {
            println!("Count: {}", num);
            let mut decrease = 5;

            loop {
                println!("Decreasing: {}", decrease);
                if decrease == 4 {
                    break;
                }
                if num == 2 {
                    break 'counter;
                }
                decrease -= 1;
            }
            num += 1; //
        }
    }
}

fn while_loops() {
    let mut num = 0;

    while num <= 5 {
        println!("Num: {}", num);
        num += 1;
    }
}

fn for_loop() {
    let vec: Vec<i8> = (0..10).collect();

    for element in vec {
        println!("{}", element);
    }

    println!("Beginning NASA TAKEOFF!!");
}

fn for_loop_2() {
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}
