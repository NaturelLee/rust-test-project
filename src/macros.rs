pub fn run() {
    println!("===macro_rules===");

    // Macros are created using the macro_rules! macro
    macro_rules! say_hello {
        () => {
            println!("hello!");
        };
    }

    say_hello!();
    // ===Syntax===

    macro_rules! create_function {
        ($func_name: ident) => {
            fn $func_name() {
                println!("You called {:?}()", stringify!($func_name));
            }
        };
    }

    create_function!(foo);
    create_function!(bar);

    foo();
    bar();

    macro_rules! print_result {
        ($expression: expr) => {
            println!("{:?} = {:?}", stringify!($expression), $expression);
        };
    }

    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });

    // The arguments of a macro are prefixed by a dollar sign $ and type annotated with a designator.
    // macro designator: ident, expr, block, item, literal, pat, path, stmt, tt, ty, vis

    // ===DSL(Domain Specific Language)===

    // ===Variadics===
}
