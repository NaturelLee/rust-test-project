pub fn main() {
  println!("this is mod formats");

  println!("{} days", 31);
  println!("{} days", 31i64);

  println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");

  println!(
    "{subject} {verb} {object}",
    object = "the object",
    verb = "the verb",
    subject = "the subject"
  );

  println!(
    "{} of {:b} people know binary, the other half doesn't",
    1, 2
  );
}
