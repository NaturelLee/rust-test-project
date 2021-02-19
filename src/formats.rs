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

    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:>0width$}", number = 1, width = 6);
    println!("my name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    println!("This is struct {:?}", Structure(3));

    let pi = 3.1415926;
    println!("Pi is roughly {:.6}", pi);
    println!("Pi is roughly {0:.1$}", pi, 6);
    println!(
        "Pi is roughly {number:.precision$}",
        number = pi,
        precision = 6
    );

    // ====Debug======
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    #[derive(Debug)]
    struct Man<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    let man = Man { name, age };

    println!("{:?}", peter);
    println!("{:?}", man);

    // ======Display=======
    use std::fmt;

    struct Onlyone(i32);

    impl fmt::Display for Onlyone {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    println!("{}", Onlyone(3));

    #[derive(Debug)]
    struct Minmax(i64, i64);

    impl fmt::Display for Minmax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minmax = Minmax(0, 14);

    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = Minmax(-300, 300);
    let small_range = Minmax(-3, 3);

    println!(
        "the big range is {big}, the small range is {small}",
        small = small_range,
        big = big_range,
    );

    let point = Point2D { x: 3.3, y: 4.4 };

    println!("Compare points:");
    println!("Debug: {:?}", point);
    println!("Display: {}", point);

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Compare Complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vet = &self.0;

            write!(f, "[")?;

            for (index, value) in vet.iter().enumerate() {
                // index != 0 前面加,
                if index != 0 {
                    write!(f, ", ")?;
                }

                write!(f, "{}: {}", index, value)?;
            }

            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("this result is: {}", v);

    // =====Formatting=====

    use std::fmt::Formatter;

    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl fmt::Display for City {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon > 0.0 { 'E' } else { 'W' };
            write!(
                f,
                "{}: {:.3}°{} {:.3}°{}",
                self.name,
                self.lat.abs(),
                lat_c,
                self.lon.abs(),
                lon_c
            )
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl fmt::Display for Color {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "RGB ({}, {}, {}) ", self.red, self.green, self.blue)?;

            write!(f, "{:#02x}{:02x}{:02x}", self.red, self.green, self.blue)
        }
    }

    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!("{}", *color);
    }
}
