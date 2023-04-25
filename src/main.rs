struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    //struct
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    // mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadow
    let y = 5;
    let y = y + 1;
    println!("The value of y is: {y}");

    // for loop
    for number in (1..4).rev() {
        println!("{number}!");
    }

    // ownership
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);    // s's value moves into the function...
    // ... s is no longer valid here
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.