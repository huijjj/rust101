fn main() {

    // branching with if expression
    let condition = true;
    if condition {
        println!("true");
    }

    // let condition = 3;
    // if condition { 
    // this yields error since rust does not try to convert 
    // non boolean to boolean like other programming languages
    // 
    // you must provide explicit boolean value for branch condition
    // }


    // since if is an expression, it can be on the right side of let statement
    let val = if condition { 5 } else { 6 };


    // repetition with loops
    // loop will loop the body block forever
    loop {
        println!("again");

        break; // will loop forever without break
    }

    // loop can be used like reduce
    let mut acc = 0;
    let val = loop {
        acc += 1;
        if acc == 2 {
            break acc; // break can be used some-what like return
        }
    };

    // also, loop can be labeled
    let mut count = 0;
    'counting_up: loop { // loop label must start with single quote
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // this will break the inner loop
            }
            if count == 2 {
                break 'counting_up; // this will break the outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // upper loops will print
    // count = 0
    // remaining = 10
    // remaining = 9
    // count = 1
    // remaining = 10
    // remaining = 9
    // count = 2
    // remaining = 10
    // End count = 2

    // looping with condition, while
    count = 0;
    'foo: while count < 3 {
        count += 1;

        // break 'foo; // not only loops with loop keyword but every loops can be labeled
    }

    // looping through collections with for
    for e in 1..4 {
        println!("{e}")
    }

}
