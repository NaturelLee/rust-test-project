// define fn in lib.rs
pub fn pformat() {
    println!("this is pformat fn defined in lib.rs");
}

// define mod/fn in lib.rs
pub mod my {
    pub fn print_format() {
        println!("this is mod my in defined in lib.rs")
    }
}

// define mod using other file
pub mod conversion;
pub mod custom_types;
pub mod expression;
pub mod flow_of_control;
pub mod formats;
pub mod functions;
pub mod primitives;
pub mod types;
pub mod variable_bindings;
