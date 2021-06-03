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
pub mod algorithms;
pub mod conversion;
pub mod custom_types;
pub mod error_handle;
pub mod expression;
pub mod flow_of_control;
pub mod formats;
pub mod functions;
pub mod generics;
pub mod macros;
pub mod misc;
pub mod primitives;
pub mod random;
pub mod rsa;
pub mod serialize;
pub mod stds;
pub mod tests;
pub mod traits;
pub mod types;
pub mod variable_bindings;
