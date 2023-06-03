#[allow(dead_code, unused_variables, unused_mut)]

fn main() {
    //* variable declaration
    let mut variable_name: i32 = 5;
    // ------------------------------------

    //* function definition
    fn sum(first: &i32, second: &i32) -> i32 {
        first + second // implicit return
    }
    // ------------------------------------

    //* pattern matching easy
    let (number, text) = (12, "Hello world!");
    // ------------------------------------

    //* pattern matching hard
    let is_two_or_three: bool = match number {
        2 | 3 => true,
        12 => false,
        _ => false,
    };
    // ------------------------------------

    //* if let
    let calculation_result = Some(5);
    if let Some(num) = calculation_result {
        // print result to standard output
        println!("Success! The result is {}", num);
    } else {
        println!("Calculation not successful");
    }
    // ------------------------------------

    //* for
    for i in 0..10 {
        println!("{}", i);
    }
    // ------------------------------------

    //* importing
    use std::collections::LinkedList;

    fn main() {
        let mut list = LinkedList::new();
        list.push_back(0);
    }
    // ------------------------------------

    //* string initialization
    let hello1: &str = "Hello world!";
    let hello2: String = "Hello world!".to_string();
    let hello3: String = String::from("Hello world!");
    // ------------------------------------
}
