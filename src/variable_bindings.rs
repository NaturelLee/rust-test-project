pub fn run() {
    println!("====variable_bindings===");
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;

    let _noisy_un_used_variable = 2u32;

    // mutability
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // _immutable_binding += 1;

    // Scope and shadowing
    let shadowned_binding = 1;
    {
        println!("before being shadowned: {}", shadowned_binding);

        // This binding *shadowed* the outer one;
        let shadowned_binding = "abc";
        println!("shadowed in inner block: {}", shadowned_binding);
    }

    println!("outside inner block: {}", shadowned_binding);

    // this binding *shadowns* the previous binding
    let shadowned_binding = 2;
    println!("shadowned in outer block: {}", shadowned_binding);

    // Freezing
    let mut _mutable_integer = 7i32;
    {
        // shadowing by immutable
        let _mutable_integer = _mutable_integer;

        // _mutable_integer = 50;
    }

    _mutable_integer = 50;
}
