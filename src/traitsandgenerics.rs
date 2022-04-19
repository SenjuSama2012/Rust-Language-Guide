#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]

pub fn sec6_traits_generics() {

    generics();
    traits();
    traits_parameters();
    drop();
    clone();
    copy();
    from_into();
    operator_overloading();
}



fn generics() {
    //--------------------------------------------------------------------------------------------------------------------
    //              -Generics
    //                  -abstract standards for concrete types
    //                  -
    //--------------------------------------------------------------------------------------------------------------------
    
    // T (The Generic) is a placeholder for the Type (Data Type) we are going to pass in there.
    struct Point<T, U> {
        x: T,   //i32
        y: U,   //i32
    }
    
    let coord = Point { x: 5.0, y: 5.0 };
    let coord2 = Point {x: 'x', y: 'y'};
    
}

fn traits() {
    //--------------------------------------------------------------------------------------------------------------------
    //              -Traits
    //                  - A capability or something that a Type can do.
    //                  - Can Be shared with other Types
    //                  - Can be used to define Shared Behavior in an Abstract Way.
    //--------------------------------------------------------------------------------------------------------------------
    
    trait Overview {
        fn overview(&self) -> String {
            String::from ("This is a Complete Guide for Rust!")
        }
    }

    struct Course {
        headline: String,
        author: String,
    }
    
    impl Overview for Course {

        fn overview(&self) -> String {
            format!("{},{}", self.author, self.headline)
        }    
    }
    struct AnotherCourse {
        headline: String,
        author: String,
    }
    
    impl Overview for AnotherCourse {}

    let course1 = Course{headline: String::from("Headline!"), author: String::from("Tanjiro!")};
    let course2 = AnotherCourse{headline: String::from("Another Headline!"), author: String::from("Inosuke!")};

    println!("{}", course1.overview());
    println!("{}", course2.overview());

}

fn traits_parameters() {
    //--------------------------------------------------------------------------------------------------------------------
    //              -Traits & Parameters
    //                  - A capability or something that a Type can do.
    //                  - Can Be shared with other Types
    //                  - Can be used to define Shared Behavior in an Abstract Way.
    //--------------------------------------------------------------------------------------------------------------------

    

}
fn drop() {}
fn clone() {}
fn copy() {}
fn from_into() {}
fn operator_overloading() {}