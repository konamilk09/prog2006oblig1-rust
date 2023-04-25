#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn count(&self) -> bool {
        self.sign_in_count > 0
    }
}

fn main() {
    // struct
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // method
    if user1.count() {
        println!("The user's sign in count is {}", user1.sign_in_count);
    }
    
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

    // refactoring tuples
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
