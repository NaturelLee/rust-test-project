#[allow(dead_code)]
pub fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0;
    let default_integer = 7;

    let mut inferred_type = 12;
    println!("inferred_type: {}", inferred_type);
    inferred_type = 234298i64;

    let mut mutalbe = 12;
    println!("mutalbe: {}", mutalbe);
    mutalbe = 22;
    println!("mutalbe: {}", mutalbe);

    // mutalbe = true;

    let mutalbe = true;

    println!("primitives: ");
    println!("logical: {}", logical);
    println!("a_float: {}", a_float);
    println!("an_integer: {}", an_integer);
    println!("default_float: {}", default_float);
    println!("default_integer: {}", default_integer);
    println!("inferred_type: {}", inferred_type);
    println!("mutalbe: {}", mutalbe);

    // ===Literals and operators===
    // Integer additon
    println!("1 + 2 = {}", 1u32 + 2);
    // integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // short circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operation
    println!("0011 AND 0101 IS {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    // ===Tuples===
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (integer, boolean) = pair;
        (boolean, integer)
    }

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);
    use std::fmt;

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "\n( {} {} )", self.0, self.1)?;
            write!(f, "( {} {} )", self.2, self.3)
        }
    }

    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);
    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // but long tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!(" pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just one integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;

    println!("{:?} {:?} {:?} {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("matrix: {}", matrix);

    fn transpose(tuple: &Matrix) -> Matrix {
        Matrix(tuple.0, tuple.2, tuple.1, tuple.3)
    }

    println!("transpose(matrix): {}", transpose(&matrix));

    // ===Arrays and Slices===
    use std::mem;
    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice is: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }

    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let ys: [i32; 50] = [0; 50];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("number of the elements in array: {}", xs.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow the whole array as a slicer");

    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // error
    // println!("{}", xs[5]);
}
