fn main() {
    another_function(3);

    // statements are instructions that perform some action and do not return a value
    let x = 5;

    // expressions evaluate to a resultant value
    let expr = {
        let x = 3;
        x + 1 // this is an expression,
        // x + 1; // this is not, this is a statement
    };
    // expression does not end with semicolon

}

fn another_function(arg: i32) {
    println!("{arg}");
}

fn implicit_return(x: i32) -> i32 {
    x // functions implicitly return last expression
}

fn return_nothing(x: i32) { // this function returns empty ()
    x; // since this is not a expression but a statement
}

fn explicit_return(x: i32) -> i32 {
    return x; // you can also return explicitly
}