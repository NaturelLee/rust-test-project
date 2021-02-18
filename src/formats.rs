pub fn main() {
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
