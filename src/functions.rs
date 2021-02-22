pub fn run() {
    println!("===functions===");
    // There's not restriction on the order of function definitions
    fizzbuzz_to(100);
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            return false;
        }
        lhs % rhs == 0
    }

    fn fizzbuzz(n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n, 5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    fn fizzbuzz_to(n: u32) {
        for n in 1..n + 1 {
            fizzbuzz(n);
        }
    }
    // === Methods ===
    /// 1. Methods are function attached to objects
    /// 2. Access object data via self keyword
    /// 3. Dedined in impl block
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        // static method
        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }

        // static method
        // Static methods don't need to be called by an instance
        // generally used as constructors
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        // This is an instance method
        // &self is sugar for self: &Self, where Self is the type of caller object; Self = Rectangle
        fn area(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        // &mut self desugars to self: &mut Self
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }

    // Pair owns two heap allocated integers
    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        fn destroy(&self) {
            let Pair(first, second) = self;
            println!("Destroying Pair({}, {})", first, second);

            // first second go out of scope and get freed
        }
    }

    let rectangle = Rectangle {
        // static methods called with double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // instance methods are called with dot operator
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // rectangle.translate(1.0, 0.0);
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    // === Closures ===
    fn function(i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };

    let closure_inferred = |i| i + 1;

    let i = 1;

    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one());
    // Capturing
    use std::mem;
    let color = String::from("green");

    let print = || println!("color: {}", color);
    print();
    let _reborrwo = &color;
    print();

    let _color_moved = color;

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };

    inc();
    // mutable borrow can only be once
    // let _reborrowed = &mut count;
    inc();

    let _count_reborrowed = &mut count;

    // non-copy type
    let movable = Box::new(3);
    let consume = || {
        println!("movalbe: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    // use of moved value;
    // consume();

    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&2));

    // borrow of moved value
    // println!("There are {} elements in vec", haystack.len());

    //  As input parameters
    // 1. Fn: the closure captures by reference(&T)
    // 2. FnMut: the closure captures by mutable reference(&mut T)
    // 3. FnOnce: the closure captures by value(T)
    // If an instance parameter is annotated as FnOnce, this specifies that the closure may capture by &T, &mut T, or T
    // This is because if a move is possible, then any type of borrow should also bu possible. The reverse is not true.
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    fn apply_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        // greeting is by reference: requires Fn
        println!("I said: {}", greeting);

        // mutation forces farewell to be captured by mutable reference. Now requires: FnMut
        farewell.push_str("!!!");
        println!("Then I screemed {}", farewell);
        println!("Now I can sleep. zzzzz");

        // manually calling drop forces farewell to be captured by value: Requries: FnOnce;
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| x * 2;

    println!("3 doubled: {}", apply_to_3(double));
    // Type anonymity
    fn applys<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }
    let x = 7;

    let print = || println!("{}", x);
    applys(print);

    // Input functions
    // As output parameters
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
        move || println!("This is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();
        move || println!("This is a: {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
    fn_plain();
    fn_mut();
    fn_once();
    // Examples in std
    // Iterator::any
    // pub trait Iterator {
    //     type Item;

    //     fn any<F>(&mut self, f: F) -> bool
    //     where
    //         F: FnMut(Self::Item) -> bool,
    //     {
    //     }
    // }

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));

    // Searching through iterators

    // === Higher Order Functions ===
    // HOF takes funtions and produce a more useful function(like: HOC);
    let upper = 1000;

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    let sum_of_squared_odd_number: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&squared| squared < upper)
        .filter(|&filterd| is_odd(filterd))
        .fold(0, |acc, n| acc + n);

    println!("--: {}", sum_of_squared_odd_number)

    // === Diverging Functioins===
}
