#! [allow(dead_code)]
#! [allow(unused_variables)]
#! [allow(unused_assignments)]
#! [allow(unused_imports)]

mod basics;
mod cargocratesmodules;
mod cliprogram;
mod commoncollections;
mod concurrency;
mod enumsandpatterns;
mod errorhandling;
mod iteratorsandclosures;
mod macros;
mod pointers;
mod rustprinciples;
mod structsandlifetimes;
mod testing;
mod traitsandgenerics;
mod unsafemethods;


fn main() {


    basics::sec2_basics();
    rustprinciples::sec3_rust_principles();
    enumsandpatterns::sec5_enums_patterns();
    traitsandgenerics::sec6_traits_generics();
    cargocratesmodules::sec7_cargo_crates_modules();
    commoncollections::sec8_common_collections();
    errorhandling::sec9_error_handling();
    testing::sec10_testing();
    cliprogram::sec11_cli_program();
    iteratorsandclosures::sec12_iterators_closures();
    pointers::sec13_pointers();
    concurrency::sec14_conccurency();
    macros::sec15_macros();
    unsafemethods::sec16_unsafe_rust(); 
    

}
