pub fn run() {
    println!("===custom_types===");

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    //  a unit struct
    struct Unit;

    // a tuple struct
    struct Pair(i32, f32);

    //  a struct with two fields
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    // Struct can be reused as fields of another struct
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    impl Rectangle {
        fn rect_area(&self) -> f32 {
            let Rectangle {
                top_left,
                bottom_right,
            } = self;

            let Point { x: x1, y: y1 } = top_left;

            let Point { x: x2, y: y2 } = bottom_right;

            ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt()
        }
    }

    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point coordinate: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };

    println!("second point: ({} {})", bottom_right.x, bottom_right.y);
    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let rectange = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    println!("Rectangle area is: {}", rectange.rect_area());

    let _unit = Unit;

    let pair = Pair(1, 0.1);
    println!("pair contain {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    fn square(p: &Point, distence: f32) -> Rectangle {
        Rectangle {
            top_left: Point {
                x: p.x - distence,
                y: p.y - distence,
            },
            bottom_right: Point { x: p.x, y: p.y },
        }
    }

    let point = Point { x: 4.0, y: 7f32 };

    println!("got a new Rect: {:?}", square(&point, 3.3));

    // ===Enum===
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed '{}'", c),
            WebEvent::Paste(s) => println!("pasted \"{}\"", s),
            WebEvent::Click { x, y } => {
                println!("clicked at x={} y={}", x, y);
            }
        }
    }

    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    let _x = Operations::Add;

    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        fn _run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    enum Status {
        Rich,
        Poor,
    }

    enum Work {
        Civilian,
        Soldier,
    }

    let status = Status::Poor;
    let work = Work::Civilian;

    match status {
        Status::Rich => println!("The rich have lots of money!"),
        Status::Poor => println!("The poor have no money..."),
    }

    match work {
        Work::Civilian => println!("Civilian work!"),
        Work::Soldier => println!("Soldier fight!"),
    }

    enum Number {
        Zero,
        One,
    }

    enum Color {
        Red = 0xff0000,
        Blue = 0x0000ff,
    }

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    // Testcase: linked-list
    enum List {
        // Cons: Tuple struct that wraps an element and a pointer to the next node
        Cons(u32, Box<List>),
        // Nil: A node that signifies the end of the linked list
        Nil,
    }

    impl List {
        fn new() -> List {
            List::Nil
        }

        fn prepend(self, element: u32) -> List {
            List::Cons(element, Box::new(self))
        }

        fn len(&self) -> u32 {
            match *self {
                List::Cons(_, ref tail) => 1 + tail.len(),
                List::Nil => 0,
            }
        }

        fn stringify(&self) -> String {
            match *self {
                List::Cons(head, ref tail) => {
                    format!("{} {}", head, tail.stringify())
                }
                List::Nil => {
                    format!("Nil")
                }
            }
        }
    }

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    // ===Constants===
    static LANGUAGE: &str = "Rust";
    const THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        n > THRESHOLD
    }

    let n = 16;
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // THRESHOLD = 5;
}
