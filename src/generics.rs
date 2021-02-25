#![allow(dead_code)]
pub fn run() {
    println!("===generics===");
    use ::std::fmt::{Debug, Display};

    // === Functions ===
    struct A;
    struct S(A);
    struct SGen<T>(T);

    fn reg_fn(_s: S) {}
    fn gen_spec_t(_s: SGen<A>) {}
    fn gen_spec_i32(_s: SGen<i32>) {}
    fn generic<T>(_s: SGen<T>) {}
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(3));
    // implicitly
    generic(SGen('a'));
    // explicitly
    generic::<char>(SGen('a'));
    generic::<i32>(SGen(3));
    generic::<f64>(SGen(3.3));
    generic::<u32>(SGen(4));

    // === Implementation ===
    #[allow(dead_code)]
    struct SA;
    struct GeneralVal<T>(T);

    impl GeneralVal<f32> {}
    impl GeneralVal<SA> {}

    // generi impl
    impl<T> GeneralVal<T> {}

    struct Val {
        val: f64,
    }

    struct GenVal<T> {
        gen_val: T,
    }

    impl Val {
        fn value(&self) -> f64 {
            self.val
        }
    }

    impl<T> GenVal<T> {
        fn value(&self) -> &T {
            &self.gen_val
        }
    }

    let x = Val { val: 3.3 };
    let y = GenVal { gen_val: 3i32 };
    println!("x: {}, y: {}", x.value(), y.value());
    // === Traits ===
    #[derive(Debug)]
    struct Empty;
    #[derive(Debug)]
    struct Null;

    trait DoubleDrop<T> {
        fn double_drop(self, _: T);
    }

    impl<T, U> DoubleDrop<T> for U {
        fn double_drop(self, _: T) {}
    }

    let empty = Empty;
    let null = Null;
    empty.double_drop(null);
    // println!("empty: {:?}, null: {:?}", empty, null);
    // === Bounds ===

    trait HasArea {
        fn area(&self) -> f64;
    }
    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.length * self.height
        }
    }

    #[derive(Debug)]
    struct Rectangle {
        length: f64,
        height: f64,
    }

    #[allow(dead_code)]
    struct Triangle {
        length: f64,
        height: f64,
    }

    // T must impl Debug
    fn print_debug<T: Debug>(t: &T) {
        println!("{:?}", t);
    }

    fn area<T: HasArea>(t: &T) -> f64 {
        t.area()
    }

    let rectangle = Rectangle {
        length: 4.0,
        height: 5.0,
    };

    let _triangle = Triangle {
        length: 4.0,
        height: 5.0,
    };

    println!("print_debug(rectangle): {:?}", print_debug(&rectangle));
    println!("area: {}", area(&rectangle));

    // println!("triangle: {:?}", print_debug(&triangle))
    // area(&triangle);

    // === Multiple Bounds ===
    fn compare_prints<T: Display + Debug>(t: &T) {
        println!("Debug: {:?}", t);
        println!("Display: {}", t);
    }

    fn compare_types<T: Debug, U: Debug>(t: &T, p: &U) {
        println!("Display: {:?}", t);
        println!("Debug: {:?}", p);
    }

    let ss = "word";
    let array = [1, 2, 3];
    let vector = vec![1, 2, 3];

    compare_prints(&ss);
    // compare_prints(&array);
    // compare_prints(&vector);
    compare_types(&array, &vector);

    // === Where clauses ===
    // === New Type Idiom ===
    // === Associated items ===
    // === Phantom type parameters ===
}
