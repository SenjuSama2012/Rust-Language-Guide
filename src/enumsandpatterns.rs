#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

pub fn sec5_enums_patterns() {
    // enums();
    // options();
    // matching();
    // if_let();
    more_matchs();
}

fn enums() {
    enum Pet {
        dog,
        cat,
        fish,
    }

    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    impl Pet {
        fn what_am_i(self) -> &'static str {
            match self {
                Pet::dog => "I am a Dog!",
                Pet::cat => "I am a Cat!",
                Pet::fish => "I am a Fish!",
            }
        }
    }

    let dog1 = Pet::dog;
    println!("{}", dog1.what_am_i());

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("My home IP Address is : {}", home.address);
    println!(
        "My business IP Address is : {}, and it's a {:?}",
        loopback.address, loopback.kind
    );
}
fn options() {
    enum Pet {
        dog,
        cat,
        fish,
    }

    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    impl Pet {
        fn what_am_i(self) -> &'static str {
            match self {
                Pet::dog => "I am a Dog!",
                Pet::cat => "I am a Cat!",
                Pet::fish => "I am a Fish!",
            }
        }
    }

    let dog1 = Pet::dog;
    println!("{}", dog1.what_am_i());

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("My home IP Address is : {}", home.address);
    println!(
        "My business IP Address is : {}, and it's a {:?}",
        loopback.address, loopback.kind
    );

    let some_number = Some(5);
    let some_string = Some("A String");
    let nothing: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    let sum = x + y.unwrap_or(0);
}
fn matching() {
    enum Pet {
        dog,
        cat,
        fish,
    }

    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    impl Pet {
        fn what_am_i(self) -> &'static str {
            match self {
                Pet::dog => "I am a Dog!",
                Pet::cat => "I am a Cat!",
                Pet::fish => "I am a Fish!",
            }
        }
    }

    let dog1 = Pet::dog;
    println!("{}", dog1.what_am_i());

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("My home IP Address is : {}", home.address);
    println!(
        "My business IP Address is : {}, and it's a {:?}",
        loopback.address, loopback.kind
    );

    //Option Lesson Begins Here
    let some_number = Some(5);
    let some_string = Some("A String");
    let nothing: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    let sum = x + y.unwrap_or(0);

    // Match Lesson Begins Here

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", none);

    fn what_pet(input: &str) {
        match input {
            "Dog" => println!("I have a dog!"),
            "Fish" => println!("I have a fish!"),
            "Cat" => println!("I have a cat!"),
            _ => println!("I have no clue what pet you have."),
        };
    }

    what_pet("Dog");
    what_pet("Cat");
    what_pet("Fish");
    what_pet("Monkey");
}
fn if_let() {
    enum Pet {
        dog,
        cat,
        fish,
    }

    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    impl Pet {
        fn what_am_i(self) -> &'static str {
            match self {
                Pet::dog => "I am a Dog!",
                Pet::cat => "I am a Cat!",
                Pet::fish => "I am a Fish!",
            }
        }
    }

    let dog1 = Pet::dog;
    println!("{}", dog1.what_am_i());

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("My home IP Address is : {}", home.address);
    println!(
        "My business IP Address is : {}, and it's a {:?}",
        loopback.address, loopback.kind
    );

    //Option Lesson Begins Here
    let some_number = Some(5);
    let some_string = Some("A String");
    let nothing: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    let sum = x + y.unwrap_or(0);

    // Match Lesson Begins Here

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", none);

    fn what_pet(input: &str) {
        match input {
            "Dog" => println!("I have a dog!"),
            "Fish" => println!("I have a fish!"),
            "Cat" => println!("I have a cat!"),
            _ => println!("I have no clue what pet you have."),
        };
    }

    what_pet("Dog");
    what_pet("Cat");
    what_pet("Fish");
    what_pet("Monkey");

    // If Let Matching Lesson Starts Here

    let dog_2 = Some(Pet::fish);
    if let Some(Pet::dog) = dog_2 {
        println!("The animal is a Dog!");
    } else {
        println!("Not a Dog!");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
fn more_matchs() {
    enum Pet {
        dog,
        cat,
        fish,
    }

    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    impl Pet {
        fn what_am_i(self) -> &'static str {
            match self {
                Pet::dog => "I am a Dog!",
                Pet::cat => "I am a Cat!",
                Pet::fish => "I am a Fish!",
            }
        }
    }

    let dog1 = Pet::dog;
    println!("{}", dog1.what_am_i());

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("My home IP Address is : {}", home.address);
    println!(
        "My business IP Address is : {}, and it's a {:?}",
        loopback.address, loopback.kind
    );

    //Option Lesson Begins Here
    let some_number = Some(5);
    let some_string = Some("A String");
    let nothing: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    let sum = x + y.unwrap_or(0);

    // Match Lesson Begins Here

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", none);

    fn what_pet(input: &str) {
        match input {
            "Dog" => println!("I have a dog!"),
            "Fish" => println!("I have a fish!"),
            "Cat" => println!("I have a cat!"),
            _ => println!("I have no clue what pet you have."),
        };
    }

    what_pet("Dog");
    what_pet("Cat");
    what_pet("Fish");
    what_pet("Monkey");

    // If Let Matching Lesson Starts Here

    let dog_2 = Some(Pet::fish);
    if let Some(Pet::dog) = dog_2 {
        println!("The animal is a Dog!");
    } else {
        println!("Not a Dog!");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // More Matches Lesson Starts Here

    let x = 1;
    match x {
        1 | 2 => println!("1 or 2"),
        _ => println!("Not 1 or 2"),
    }

    // Match on Inclusive Ranges of Values

    match x {
        1..=5 => println!("Matches"), //Add "=" to range to check for matches
        _ => println!("Not Matching"),
    }

    //If Condition specified after the pattern in a MAtch

    let x = Some(7);
    let y = 5;

    match x {
        Some(10) => println!("10"),
        Some(x) if x == y => println!("Matches"),
        _ => println!("Default"),
    }
}
