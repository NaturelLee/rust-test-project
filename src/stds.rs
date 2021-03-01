pub fn run() {
    println!("===Std Library types ===");

    // === Box, stack and heap ===
    use std::mem;

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: f64,
        y: f64,
    }

    #[allow(dead_code)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn boxed_origin() -> Box<Point> {
        Box::new(Point { x: 0.0, y: 0.0 })
    }

    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    let boxed_point: Box<Point> = Box::new(origin());

    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!(
        "Point occupies {} bytes on the stack",
        mem::size_of_val(&point)
    );
    println!(
        "Rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rectangle)
    );

    println!(
        "Boxed point occupies {} bytes on the stack",
        mem::size_of_val(&boxed_point)
    );
    println!(
        "Boxed rectangle occupies {} bytes on the stack",
        mem::size_of_val(&boxed_rectangle)
    );

    println!(
        "Boxed box occupies {} bytes on  the stack",
        mem::size_of_val(&box_in_a_box)
    );

    let unboxed_point: Point = *boxed_point;
    println!(
        "Unboxed point occupies {} bytes on the stack",
        mem::size_of_val(&unboxed_point)
    );

    // === Vectors ===
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("collectd (0..10) into: {:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    println!("Vector length: {}", xs.len());
    println!("Second element: {:?}", xs[1]);

    println!("Pop last element:{:?}", xs.pop());

    // println!("Fourth element: {}", xs[3]);

    println!("Contents of xs");

    for x in xs.iter() {
        println!("> {}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);
    // === Strings ===
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    println!("Words in reverse");

    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    let mut string = String::new();

    for c in chars {
        string.push(c);

        string.push_str(", ");
    }

    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    let alice = String::from("I like dogs");
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);

    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3f means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (u+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals
            can span multiple lines.
            The linebreak and indentation here ->\
            <- can be escaped too!";
    println!("{}", long_string);

    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    print!("{}", raw_str);

    let quotes = r#"And the I said: "There is no escape!""#;
    println!("{}", quotes);

    let longer_delimiter = r###"A string with "# in it. And even "##!"###;

    print!("{}", longer_delimiter);
    // === Option ===

    // === Result ===
    // === panic! ===
    // === HashMap ===
    use std::collections::HashMap;

    fn call(number: &str) -> &str {
        match number {
            "798-1364" => {
                "We're sorry, the call cannot be completed as dialed.
            Please hang up and try again."
            }
            "645-7689" => {
                "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?"
            }
            _ => "Hi! Who is this again?",
        }
    }

    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => print!("Don't have Daniel's number"),
    }

    contacts.insert("Daniel", "164-6743");

    contacts.remove(&"Ashley");

    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }
    // === Rc ===
    use std::rc::Rc;
    {
        let rc_examples = "Rc examples".to_string();
        {
            println!("--- rc_a is created --");
            let rc_a: Rc<String> = Rc::new(rc_examples);
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
            {
                println!("--- rc_a is cloned to rc_b ---");

                let rc_b: Rc<String> = Rc::clone(&rc_a);
                println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
                print!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

                println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

                println!("Length fo the value inside rc_a: {}", rc_a.len());
                println!("Value of rc_b:{}", rc_b);
                println!("---rc_b is dropped out of scope");
            }
            println!("Reference COunt of rc_a is {}", Rc::strong_count(&rc_a));
            println!("----rc_a is dropped out of scope");
        }
        // print!("rc_examples: {}", rc_examples);
    }
    // === Arc ===
    {
        use std::sync::Arc;
        use std::thread;

        let apple = Arc::new("the same apple");

        for index in 0..10 {
            let apple = Arc::clone(&apple);

            thread::spawn(move || {
                println!("{:?} - {}", apple, index);
            });
        }
    }
}
