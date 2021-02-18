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
pub mod formats;
