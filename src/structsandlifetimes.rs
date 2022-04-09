#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]

pub fn sec4_structs_lifetimes() {

    structs();
    methods();
    lifetimes();
    lifetimes_structs();
    static_lifetimes();

}

fn structs() {
    //      Structs
    //          -Structs use Camel Case (Haskell Case??) naming conventions
    //          -Allow you to name and package together related values into a single value.
    //              -This allows you to deal them as a single unit
    //          -Three Types of Structs
    //              -Field Struct
    //                  -Gives a name to each component
    //              -Tuple Struct
    //                  -Identifies values by the order which they appear
    //              -Unit Struct
    //                  -No Components
    //          -
    //          -
    //------------------------------------------------------------------------------------------------
    
    //          -Field Struct Example-
    struct User {
        active: bool,
        username: String,
        sign_in_count: u32
    }
    
    let user1 = User{active: true, username: String::from("Dekum"), sign_in_count: 0};
    println!("{}", user1.username);
    
    let user2 = build_user(String::from("Tanjiro"));
    println!("{}", user2.username);
    
    fn build_user(username: String) -> User {
        User {
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    
    //------------------------------------------------------------------------------------------------
    //          -Tuple Struct Example-
    
    struct Coordinates(i32,i32,i32);
    
    let cords = Coordinates(1,2,3);
    
    //------------------------------------------------------------------------------------------------
    //          -Unit Struct Example-
    //              -Range is an example of a struct within Rust
    //                  -Ex: ..., 1..5 , and Range {start: 1, end: 5}
    
    

}
fn methods() {
    //      Methods(similar to functions)
    //          -Methods are defined within the context of a struct, enum and trait object
    //          -Methods ALWAYS have their 1st parameter as "Self", which is representing the isntance of the Struct
    //           
    //------------------------------------------------------------------------------------------------


    struct Square {
        width: u32,
        height: u32,
    }

    impl Square {
        fn area(&mut self) -> u32 {
            self.width * self.height
        }

        fn whats_my_width(&self) -> u32 {
            self.width
        }

        fn change_my_width(&mut self, new_width: u32) {
            self.width = new_width;
        }
    }

    let mut sq = Square {width: 5, height: 5};
    println!("{}", sq.area());
    println!("{}", sq.whats_my_width());

    sq.change_my_width(10);
    println!("{}", sq.whats_my_width());
}

fn lifetimes(){
    //      Lifetimes(similar to functions)
    //          -Every reference has a lifetime
    //          -Lifetimes are Implicit and Inferred
    //          -These rules prevent Dangling References
    //          -
    //          -
    //------------------------------------------------------------------------------------------------

    let r;
    {
        let x = 5;
        r = x; //This throws error due to Lifetime being dropped.
    }

    println!("{}", r); //'a

    //&i32
    //&'a i32
    // &'a mute i32

    fn example <'a>(x: &'a str) -> &'a str {
        x
    }
    
    //'a for one parameter, 'b for second parameter
    
    //Example of why we need Lifetime Annotations.
    fn example_2 <'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
        x
    }
}

fn lifetimes_structs() {

    struct MyString<'a> {
        text: &'a str,
    }

    let str1 = String::from("This is my String");
    let x = MyString{text: str1.as_str()};



}
fn static_lifetimes() {
    // Static Lifetimes
    //          -Static Lifetimes live for the duration of the Program
    //          -Sting Literals all have static lifetimes.
    //              -The Static Liftimes is stored directly in the programs binary
    //          -Most Lifetime Errors happen from Dangling References.
    
    let s: &'static str = "I have a static lifetime.";

}