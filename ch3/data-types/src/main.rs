fn main() {
    // scalar types
    let int: u32 = 1_000; // same as 1000
    
    let double = 3.1; // f64
    let single: f32 = 2.0; // f32

    let div = double / single; // div operator with floating points work as division
    let trunc = int / 133; // integer division truncates towards zero to the nearest integer


    // compound types
    let tup = (500, 1.2, 1);
    // tuple elements can be accessed directly with . operator, e.g. tup.0 == 500

    let unit = (); // empty tuple represents an empty value or empty return type
    // expressions implicitly return unit value if they do not return any other value

    let arr = [1, 2, 3, 4];
    let arr_with_same_val = [3; 5]; // array of length 5 filled with 3
    // array elements can be accessed with [] operator, e.g. arr[0] == 1
}
