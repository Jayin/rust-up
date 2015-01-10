fn main() {
    println!("Hello world!");

    let x: i32 = 273942569;//默认不可变
    //let mut x :i32 = 273942569 //这样就是可变变量
    println!("The value of x is: {}", x);

    // if
    let x = 5;
    if x == 5 {
        println!("x is five!");
    } else {
        println!("x is not five :(");
    };

    let y =  if x == 5 { 10 } else { 15 };
    println!("{}",y);
    print_number(y);
    print_number(add_one(y));
    print_number(foo(y));
    
    cdt();

    match_test();

    looping_test();
}

// --------------Function example-----------

//NOTE:必须声明函数参数类型
fn print_number(x: i32) {
    println!("x is: {}", x);
}


fn add_one(x: i32) -> i32 {
    x + 1
    //NOTE : 尾值返回不需要分号
    //Reason: 
    //   Remember our earlier discussions about semicolons and ()?
    //   Our function claims to return an i32, but with a semicolon, it would return () instead. 
    //   Rust realizes this probably isn't what we want, and suggests removing the semicolon.
}

fn foo(x: i32) -> i32 {
    if x < 5 { return x; } // here requires ; 

    x + 1
}

//---------- Compound Data Types------------

fn cdt(){
    // Tuples

    let x = (1, "hello");
    // Tuples with type annotated
    //let x: (i32, &str) = (1, "hello");


    //Structs
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 }; // origin: Point

    println!("The origin is at ({}, {})", origin.x, origin.y);

    //Enums
    enum Ordering {
        Less,
        Equal,
        Greater,
    }
}

//--------Match
fn match_test(){
    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    };
}

//--------Looping-----
fn looping_test(){
    for x in range(0, 10) {
        println!("{}", x); // x: i32
    }


    let mut x = 5u;       // mut x: uint
    let mut done = false; // mut done: bool

    while !done {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { done = true; }
    }


    // loop instead of while done
    let mut x = 5u;
    loop {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { break; }
    }

}