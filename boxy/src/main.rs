// box is the most straight forward smart pointer
// and probably the most useful one
// when wrapped in a box, primitive will be allocated to heap


// cons list is data structure from lisp programming language
// in lisp, there's function called cons, which can be called
// to create a new list. teh list is just a recursive call on the
// cons operator until it creates a list. lol.

enum List {
    Cons(i32, Box<List>),
    End,
}

use List::Cons;
use List::End;

fn main() {
    // creates list
    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(End))))));

    let y = 4;
    let x = &y;
    let z = Box::new(y);

    // becomes more useful with closures
    if *x = *y {
        println!("True");
    }
}

// closures are anonymous functions that can be saved as a variable
// and pass as arguments to other functions
// and you can even have a function that creates a closure
//
// usually we use the fn macro and the name of the function
// then we put in the arg with the type declaration and return type
// then function bod in brackets
// can create anonymous one with diff format-- with pipe operators
// in place of parentheses
// don't need type

fn diff() {
    let f = |i| i + 1;
    let x = 10;

    println!("{}", f(x));
    let p = || println!("this is a closure");
    p();
}

// closures are inherently flexible and will do what the functionality requires
// to make the closure work without annotation
// this allows capturing to reflexively adapt to the use case,
// sometimes moving, sometimes borrowing
// closure can capture vars in various diff ways
// they have PREFERENCE to capture by reference,
// which makes it easier for us in Rust

fn example() {
    let mut c = 0;
    let mut inc = || {
        c += 1;
        println!("incremented by 1: {}", c);
    };

    inc();
    inc();
    inc();
}
// this function borrows c from the outer scope

fn run<F>(f: F)
where F: Fn() {
    f();
}

fn add3<F>(f: F) -> i32
where F: Fn(i32) -> i32 {
    f(3)
}

struct A<F: Fn(i32) -> i32 {
    f: F
}

fn little_closure() {
    let p = || println!("hello from run function!");
    run(p);

    let x = |i| i * 10;

    // lets us store x closure in this struct
    let a = A {
        f: x,
    };
}

///////

fn create() -> Box<Fn()> {
    Box::new(move || println!("this is a closure in a box!"))
}
// as soon as we exit, the values would die from the scope, but
// by using the move keyword, we prevent this from happening
// we turn 'x' into CREATE

fn main() {
    let x = create();
    x();
}


/////////
 fn main() {
     let v = vec![1, 2, 3];
     println!("v {}", v.iter().any(|&x| x != 2));
 }
// this comes back as true because not equal to two
// iterators for task over sequence of items
// associated type called item, which is type of actual collection that we're
// trying to iterate through
// when we iterate through this vector
