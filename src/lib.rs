// define fn in lib.rs
pub fn pformat() {
  println!("{} days", 31);
  println!("{} days", 31i64);

  println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");

  println!(
    "{subject} {verb} {object}",
    object = "the object",
    verb = "the verb",
    subject = "the subject"
  )
}

// define mod/fn in lib.rs
pub mod my {
  pub fn print_format() {
    println!("{} days", 31);
    println!("{} days", 31i64);

    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");

    println!(
      "{subject} {verb} {object}",
      object = "the object",
      verb = "the verb",
      subject = "the subject"
    )
  }
}

// define mod using other file
pub mod formats;
