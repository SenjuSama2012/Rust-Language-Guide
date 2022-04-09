#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]

pub fn sec3_rust_principles() {
    ownership();
    ownership_move();
    ownership_clone();
    ownership_copy();
    ownership_more_moves();
    ownership_referencing_borrowing();
}

fn ownership() {
    //Will be on the Stack
    let var = 1;
    //Once this function "Ownership" is done, "var" will be freed/removed from the memory in the Stack.

    //Will be on the Heap
    let mut s = "Hello".to_string();
    s.push_str(" , world!");
}
fn ownership_move() {
    let x = vec!["tyler".to_string()];
    let y = x;
    let z = y;
    //UNCOMMENT BELOW PRINT STATEMENT TO SEE COMPILER
    // println!("{:?}", x);
    //Error occurs because we already moved the value of x to y before. Look BELOW!

    /*                  *Compiler That Will Happen!*
    //------------------------------------------------------------------------------------
    |
    31 |     let x = vec!["tyler".to_string()];
    |         - move occurs because `x` has type `Vec<String>`, which does not implement the `Copy` trait
    32 |     let y = x;
    |             - value moved here
    33 |     println!("{:?}", x );
    |                      ^ value borrowed here after move

    //------------------------------------------------------------------------------------
    */
}
fn ownership_clone() {
    let x = vec!["tyler".to_string()];
    let y = x.clone();
    let z = y.clone();
    //These are Deep Copies.
    //  -This Clone is an expensive way to implement.
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}
fn ownership_copy() {
    let x = 1;
    let y = x;
    println!("X = {}, Y = {}", x, y);
}
fn ownership_more_moves() {
    //Created a Variable with a String Takes
    let s = String::from("Takes");

    //Give Ownership to the Function
    takes_ownership(s);

    let value = 1;
    make_copy(value);

    let str1: String = give_ownership();
    println!("{}", str1);

    let str3: String = take_give_ownership(str1);
    // println!("{}", str1);

    if true {
        let str4 = str3;
    } else {
        let str5 = str3;
    }

    let mut string1 = String::from("Tyler");
    let mut string2: String;

    //Ownership Error occurs EVEN IN LOOPS.
    // loop {
    //     string2 = string1;
    // }

    fn takes_ownership(s: String) {
        let string = s;
    }

    fn make_copy(one: i32) {
        let val1 = one;
        println!("{}", val1);
    }

    fn give_ownership() -> String {
        "given".to_string()
    }

    fn take_give_ownership(str2: String) -> String {
        str2
    }
}
fn ownership_referencing_borrowing() {
    // Two Types References
    //      -Shared
    //          -Allows you to read but NOT MODIFY whats being reference
    //          -You can Reference as much as you want to a value
    //      -Immutability
    //          -Allow you to Read AND Modify the value
    //          -You CANNOT have multiple active refeences at the same time.
    //      -
    //      -

    fn shared_reference() {
        println!("This function needs to be reviewed!");
        // Issues with Static Lifetime Error

        // let s = String::from("Hello");
        // change_string(s);
        // println!("{}", s);

        // fn change_string(some_string: String) -> &mut String {
        //     some_string.push_str(", world!");
        // }
    }
}
