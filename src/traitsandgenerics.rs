#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::ops::Add;

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
    //                  -They Allow us to have stand in Types for our Concrete Types
    //                  -They also allow our code to be able to operate on many different types.
    //--------------------------------------------------------------------------------------------------------------------

    // T (The Generic) is a placeholder for the Type (Data Type) we are going to pass in there.
    struct Point<T, U> {
        x: T, //i32
        y: U, //i32
    }

    let coord = Point { x: 5.0, y: 5.0 };
    let coord2 = Point { x: 'x', y: 'y' };
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
            String::from("This is a Complete Guide for Rust!")
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

    let course1 = Course {
        headline: String::from("Headline!"),
        author: String::from("Tanjiro!"),
    };
    let course2 = AnotherCourse {
        headline: String::from("Another Headline!"),
        author: String::from("Inosuke!"),
    };

    println!("{}", course1.overview());
    println!("{}", course2.overview());
    println!("Lesson Ends Here \n");
}

fn traits_parameters() {
    //--------------------------------------------------------------------------------------------------------------------
    //              -Traits & Parameters
    //                  - Traits can be passed as Parameters/Arguments as well.
    //
    //--------------------------------------------------------------------------------------------------------------------
    trait Overview {
        fn overview(&self) -> String {
            String::from("This is a Complete Guide for Rust!")
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

    let course1 = Course {
        headline: String::from("Headline!"),
        author: String::from("Tanjiro!"),
    };
    let course2 = AnotherCourse {
        headline: String::from("Another Headline!"),
        author: String::from("Inosuke!"),
    };
    
    println!("{}", course1.overview());
    println!("{}", course2.overview());
    println!("***Lesson Ends Here*** \n");
    
    //------------------------------------------------------------------------------------------------
    //Lesson Starts Here for Parameters
    println!("\n ***Lesson Starts Here for Trait Parameters***");
    
    fn call_overview<T: Overview>(item: &T) {
        println!("Overview: {}", item.overview())
    }
    
    call_overview(&course1);
    call_overview(&course2);

    //Trait Bound
    // Example: fn overview(item1: &impl Overview, item2: &impl Overview)
    //fn overview<T: Overview>(item1: &T, item2: &T)

    //Multiple Trait Bounds
    //fn overview(item1: &impl Overview + AnotherTrait)
    //fn overview<T: Overviw + AnotherTrait(item1: &T, item2: &T)

    println!("***Lesson Ends Here*** \n");
    
    
    
    
}
fn drop() {
    //--------------------------------------------------------------------------------------------------------------------
    //              -Utility Traits & Drop
    //                  - Drop Traits
    //                      - 
    //                  - 
    //
    //--------------------------------------------------------------------------------------------------------------------
    println!("\n ***Lesson Starts Here for Drop Trait***");
    trait Overview {
        fn overview(&self) -> String {
            String::from("This is a Complete Guide for Rust!")
        }
    }
    struct Course {
        headline: String,
        author: String,
    }

    impl Drop for Course {
        fn drop(&mut self) {
            println!("Dropping: {}", self.author)
        }
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

    let course1 = Course {
        headline: String::from("Headline!"),
        author: String::from("Tanjiro!"),
    };

    let course2 = AnotherCourse {
        headline: String::from("Another Headline!"),
        author: String::from("Inosuke!"),
    };



    // fn call_overview<T: Overview>(item: &T) {
    //     println!("Overview: {}", item.overview())
    // }
    
    println!("***Lesson Ends Here*** \n");
}
fn clone() {

//Clone Explained from the Standard Library. CODE BELOW AS EXAMPLE

//     trait Clone: Sized {
//         fn clone(&self) -> Self;
//         fn flone_from(&mut set, source: &Self) {
//             &self = source.clone();
//         }
//     }
}
fn copy() {

    //Informative Lesson
}
fn from_into() {
    //--------------------------------------------------------------------------------------------------------------------
    //              -From & Into
    //                  - From
    //                      -Performs conversion
    //                  -Into
    //                      -Into
    //                  -TryFrom & TryInto
    //                      -Error Catching forms of From and Into
    //
    //--------------------------------------------------------------------------------------------------------------------
}
fn operator_overloading() {

    #[derive(Debug)]
    struct Point2<T> {
        x: T,
        y: T,
    }

    impl<T> Add for Point2<T> 
        where
        T: Add<Output = T> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Point2 {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }


    let coordinate1 = Point2 {x: 5.0, y : 5.0};
    let coordinate2 = Point2 {x: 1.0, y: 2.0};

    let sum = coordinate1 + coordinate2;

    println!("{:?}", sum);

}
