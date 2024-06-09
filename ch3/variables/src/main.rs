fn main() {
    let x = 5;
    // x = 6; // this will yield compile error
    // Compiling variables v0.1.0 (/Users/jeonghuijong/rust101/ch3/variables)
    // warning: value assigned to `x` is never read
    //  --> src/main.rs:2:9
    //   |
    // 2 |     let x = 5;
    //   |         ^
    //   |
    //   = help: maybe it is overwritten before being read?
    //   = note: `#[warn(unused_assignments)]` on by default
    
    // error[E0384]: cannot assign twice to immutable variable `x`
    //  --> src/main.rs:3:5
    //   |
    // 2 |     let x = 5;
    //   |         -
    //   |         |
    //   |         first assignment to `x`
    //   |         help: consider making this binding mutable: `mut x`
    // 3 |     x = 6; // this will yield compile error
    //   |     ^^^^^ cannot assign twice to immutable variable
    
    // For more information about this error, try `rustc --explain E0384`.
    // warning: `variables` (bin "variables") generated 1 warning
    // error: could not compile `variables` (bin "variables") due to 1 previous error; 1 warning emitted

    let mut mutable = 5; // keyword mut should be used for mutable variable
    mutable = 3; // this is OK since mutable is declared with mut keyword
    println!("{mutable}");

    // constants can be declared with const keyword
    const HOUR_IN_SECONDS: u32 = 60 * 60; // note that const must be declared with a data type

    // variable shadowing
    let x = 5; // x == 5
    let x = x + 1; // x == 6
    {
        let x = x * 2; // x == 12
    }
    // x == 6

}
