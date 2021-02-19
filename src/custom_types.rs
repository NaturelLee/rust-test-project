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
}
