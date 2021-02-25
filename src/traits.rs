pub fn run() {
    println!("===trait==");
    // === Returning Traits with dyn ===
    struct Sheep {}
    struct Cow {}

    trait Animal {
        fn noise(&self) -> &'static str;
    }

    impl Animal for Sheep {
        fn noise(&self) -> &'static str {
            "Blablabla"
        }
    }

    impl Animal for Cow {
        fn noise(&self) -> &'static str {
            "moooooooo"
        }
    }

    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }

    let random_num = 0.8;

    let animal = random_animal(random_num);
    println!(
        "You've randomly chosen an animal, and it says: {}",
        animal.noise()
    );
    // === Operator Overloading ===
    use std::ops;
    struct Foo;
    struct Bar;

    #[derive(Debug)]
    struct FooBar;

    #[derive(Debug)]
    struct BarFoo;

    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            println!("> Foo.add(Bar) was called");

            FooBar
        }
    }

    impl ops::Add<Foo> for Bar {
        type Output = BarFoo;

        fn add(self, _rhs: Foo) -> BarFoo {
            println!("> Bar.add(Foo) was called");

            BarFoo
        }
    }

    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);

    // === Drop ===
    struct Droppable {
        name: &'static str,
    }

    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> Dropping {}", self.name);
        }
    }

    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };

            let _d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Just Exited block B");

        println!("Exiting block A");
    }

    drop(_a);
    println!("end of the drop");
    // === Iterators ===
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            let new_next = self.curr + self.next;

            self.curr = self.next;
            self.next = new_next;

            Some(self.curr)
        }
    }

    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }

    let mut sequence = 0..3;
    println!("Four consecutive next calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterator through 0..3 using for");
    for i in 0..3 {
        println!("> {}", i);
    }

    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    println!("Iterator the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
    // === Impl Trait ===
    use std::iter;
    use std::vec::IntoIter;

    fn _combine_vecs_explicit_return_type(
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    assert_eq!(Some(6), v3.next());
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    assert_eq!(Some(6), v3.next());
    println!("all done");

    fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
        let closure = move |x: i32| x + y;
        closure
    }

    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);

    fn _double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
        numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
    }

    // === Clone ===

    // === Supertraits ===
    trait Person {
        fn name(&self) -> String;
    }

    trait Student: Person {
        fn university(&self) -> String;
    }

    trait Programmer {
        fn fav_language(&self) -> String;
    }

    trait ComSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }

    fn _comp_sci_student_greeting(student: &dyn ComSciStudent) -> String {
        format!(
            "My name is {}, and I attend {}, My favorite language is {}. My git user name is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username(),
        )
    }

    // === Disambiguating overlapping traits ===
    trait UsernameWidget {
        fn get(&self) -> String;
    }

    trait AgeWidget {
        fn get(&self) -> u8;
    }

    struct Form {
        username: String,
        age: u8,
    }

    impl UsernameWidget for Form {
        fn get(&self) -> String {
            self.username.clone()
        }
    }

    impl AgeWidget for Form {
        fn get(&self) -> u8 {
            self.age
        }
    }

    let form = Form {
        username: "rustacean".to_owned(),
        age: 30,
    };

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);

    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(30, age);
}
