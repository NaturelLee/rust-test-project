pub fn run() {
    println!("====Flow of Control====");
    // 1. if/else
    let n = 5;
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);
    // 2. loop
    let mut count = 0u32;
    println!("Let's count until infinity");

    loop {
        count += 1;

        if count == 3 {
            println!("Three!");

            // skip
            continue;
        }

        println!("{}", count);

        if count == 10 {
            println!("OK, that's enough");
            // break the loop;
            break;
        }
    }

    // 2.1 nesting and labels
    'out: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            // this would break only the inner loop;
            // break;
            break 'out;
        }

        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    // 2.2 returning from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {}", result);

    // 3. while
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
    // 4. for and range
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // the above can be written as:
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];

    // names can be reused after iteration
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    // names can not be reused after iteration
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    let mut names1 = vec!["Bob", "Frank", "Ferris"];

    // this will change names1
    for name in names1.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names1);

    // 5. match
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} - {}", boolean, binary);

    // destructure
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("First is 0, y is {:?}, and z is {:?}", y, z),
        (1, ..) => println!("First is 1 and the rest doesn't matter"),
        _ => println!("It doesn't matter"),
    }

    #[allow(dead_code)]
    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);

    println!("what color is it?");

    match color {
        Color::Red => println!("The color is Red"),
        Color::Blue => println!("The color is Blue"),
        Color::Green => println!("The color is Green"),
        Color::RGB(r, g, b) => {
            println!("Red: {}, green: {}, blue: {}", r, g, b);
        }
        Color::HSV(h, s, v) => {
            println!("Hue: {}, Saturation: {}, value: {}", h, s, v);
        }
        Color::HSL(h, s, l) => {
            println!("Hue: {}, saturation: {}, lightness: {}", h, s, l);
        }
        Color::CMY(c, m, y) => {
            println!("Cyan: {}, magenta: {}, yellow: {}", c, m, y);
        }
        Color::CMYK(c, m, y, k) => {
            println!(
                "Cyan: {}, magenta: {}, yellow: {}, key(black): {}",
                c, m, y, k
            );
        }
    }

    // pointers/ref
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;

    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10, mut_value: {:?}", m);
        }
    }

    // structs
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (2, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1,  b = {}, y = {}", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }

    // 5.2 Guards
    let pair = (2, 2);

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, y) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correction..."),
    }

    // 5.3 binding
    fn age() -> u32 {
        15
    }

    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    fn some_number() -> Option<u32> {
        Some(42)
    }

    match some_number() {
        Some(n @ 42) => println!("The Answer: {}", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }

    // 6. if let

    let number = Some(7);
    if let Some(i) = number {
        println!("Matched: {:?}", i);
    }

    let letter: Option<i32> = None;
    if let Some(i) = letter {
        println!("Matched: {:?}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }
    let i_like_letters = false;
    let emotion: Option<i32> = None;

    if let Some(i) = emotion {
        println!("Matched: {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number, Let's go with a letter");
    } else {
        println!("I don't like letters, Let's go with an emotion :)!");
    }

    enum Foos {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foos::Bar;
    let b = Foos::Baz;
    let c = Foos::Qux(100);

    if let Foos::Bar = a {
        println!("a is foobar");
    }

    if let Foos::Baz = b {
        println!("b is foobar");
    }

    if let Foos::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foos::Qux(value @ 100) = c {
        println!("c is 100");
    }

    enum Foo2 {
        Bar,
    }
    let a = Foo2::Bar;
    if let Foo2::Bar = a {
        println!("a is foobar");
    }

    // 7. while let
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("i is {:?}, Try again", i);
            optional = Some(i + 1);
        }
    }
}
