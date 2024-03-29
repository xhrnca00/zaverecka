#[allow(dead_code, unused_variables, unused_mut, unused_assignments)]

fn main() {
    {
        //* variable declaration
        let mut variable_name: i32 = 5;
        // ------------------------------------
    }
    {
        //* function definition
        fn sum(first: &i32, second: &i32) -> i32 {
            first + second // implicit return
        }
        // ------------------------------------
    }
    {
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
    }
    {
        //* if let
        let calculation_result = Some(5);
        if let Some(num) = calculation_result {
            // print result to standard output
            println!("Success! The result is {}", num);
        } else {
            println!("Calculation not successful");
        }
        // ------------------------------------
    }
    {
        //* for
        for i in 0..10 {
            println!("{}", i);
        }
        // ------------------------------------
    }
    {
        //* importing
        use std::collections::LinkedList;

        fn main() {
            let mut list = LinkedList::new();
            list.push_back(0);
        }
        // ------------------------------------
    }
    {
        //* vector and array initialization
        let mut array: [i32; 3] = [0; 3];
        array[0] = 1;
        let vector1: Vec<i32> = Vec::new();
        let vector2: Vec<i32> = Vec::with_capacity(10);
        let mut vector = vec![1, 2, 3];
        vector.push(4);
        // ------------------------------------
    }
    {
        //* references and slices
        fn change_number(number: &mut u64) {
            *number = 0;
        }

        fn main() {
            let mut number = 5;
            change_number(&mut number);
            println!("{}", number); // 0
            let mut vector = vec![1, 2, 3];
            let number_slice: &mut [i32] = &mut vector[1..];
            number_slice[1] = 0;
            println!("{}", vector[2]); // 0
        }
        // ------------------------------------
        main();
    }
    {
        //* string initialization
        let hello1: &str = "Hello world!";
        let hello2: String = "Hello world!".to_string();
        let hello3: String = String::from("Hello world!");
        // ------------------------------------
    }
    {
        //* struct definition
        struct Person {
            name: String,
            age: u8,
        }
        // ------------------------------------

        //* impl blocks
        impl Person {
            fn new(name: &str, age: u8) -> Person {
                Person {
                    name: name.to_string(),
                    age, // equivalent to `age: age`
                }
            }

            fn add_year(&mut self) {
                self.age += 1;
            }
        }
        // ------------------------------------
    }
    {
        //* enum definition and impl
        enum FileError {
            // An `enum` variant may either be `unit-like`,
            NotFound,
            // like tuple structs,
            TooBig(u64),
            // or c-like structures.
            DifferentOwner { user: String },
        }

        impl FileError {
            fn description(&self) -> String {
                match self {
                    FileError::NotFound => "File not found".to_string(),
                    FileError::TooBig(max_size) => {
                        format!("File too big, max size is {}", max_size)
                    }
                    FileError::DifferentOwner { user } => {
                        format!("Different owner: {}", user)
                    }
                }
            }
        }
        // ------------------------------------

        let error = FileError::TooBig(1024);
        println!("{}", error.description());
        if let FileError::TooBig(value) = error {
            println!("File too big, max size is {}", value);
        }
    }
    {
        //* move semantics
        let a = vec![1, 2, 3];
        let b: Vec<i32> = a; // move `a` into `b`
                             // for elem in a {
        for elem in b {
            println!("{}", elem);
        }
        // ------------------------------------
    }
    {
        //* trait usage
        use std::f64::consts::PI;

        trait Shape {
            fn area(&self) -> f64;
        }

        struct Rectangle {
            width: f64,
            height: f64,
        }

        struct Circle {
            radius: f64,
        }

        impl Shape for Rectangle {
            fn area(&self) -> f64 {
                self.width * self.height
            }
        }

        impl Shape for Circle {
            fn area(&self) -> f64 {
                self.radius * self.radius * PI
            }
        }
        // ------------------------------------
        let rectangle = Rectangle {
            width: 2.0,
            height: 3.0,
        };
        let circle = Circle { radius: 2.0 };
        //* generics
        fn print_area_generic1<T: Shape>(shape: &T) {
            println!("This shape has an area of {:.2}", shape.area());
        }

        fn print_area_generic2<T>(shape: &T)
        where
            T: Shape,
        {
            println!("This shape has an area of {:.2}", shape.area());
        }
        // ------------------------------------
        //* trait objects
        fn print_area_tobj(shape: &dyn Shape) {
            println!("This shape has an area of {:.2}", shape.area());
        }
        // ------------------------------------
        print_area_generic1(&rectangle);
        print_area_generic2(&rectangle);
        print_area_tobj(&rectangle);
        print_area_generic1(&circle);
        print_area_generic2(&circle);
        print_area_tobj(&circle);
    }
    {
        //* generic impl
        trait Average: Iterator<Item = usize> {
            fn average(mut self) -> f64
            where
                Self: Sized,
            {
                let mut sum = 0;
                let mut count: usize = 0;
                for value in self {
                    sum += value;
                    count += 1;
                }
                // will be NaN if the iterator is empty
                sum as f64 / count as f64
            }
        }

        impl<T: Iterator<Item = usize>> Average for T {}

        fn main() {
            let v = vec![1, 2, 3];
            println!("{}", v.into_iter().average());
        }
        // ------------------------------------
        main();
    }
    {
        //* format! macro and strings
        let name: &str = "John";
        let greeting: String = format!("Hi {}!", name); // Hi John!
        let some_numbers: Vec<i32> = vec![1, 2, 3];
        let debug: String = format!("{some_numbers:?}"); // [1, 2, 3]
        for c in greeting.chars() {
            // print char by char
            print!("{c}");
        }
        println!(); // end with newline
        println!("{debug}");
        // ------------------------------------
    }
    {
        //* borrowing
        let mut value = 5;
        let reference = &value;
        // let mut mutable_reference = &mut value;
        println!("{}", reference);
        // ------------------------------------
    }
    {
        //* invalid reference
        let value = 5;
        let mut reference: &u64 = &value;
        {
            let another = 30;
            // reference = &another;
        }
        println!("{}", reference);
    }
    {
        //* smart pointers
        use std::rc::{Rc, Weak};

        fn main() {
            let boxed_value: Box<i32> = Box::new(5);
            let rc_value: Rc<i32> = Rc::new(*boxed_value);
            let strong_reference: Rc<i32> = Rc::clone(&rc_value);
            let weak_reference: Weak<i32> = Rc::downgrade(&rc_value);
            println!("{}", Rc::strong_count(&rc_value)); // 2
            println!("{}", Rc::weak_count(&rc_value)); // 1
        }
        // ------------------------------------
        main();
    }
    {
        //* panic
        fn main() {
            panic!("crash and burn");
        }
        // ------------------------------------
        // main(); // we know what this does
    }
    {
        //* result
        let result: Result<i32, String> = Ok(200);
        // match
        match &result {
            Ok(code) => println!("Code: {}", code),
            Err(error) => println!("Error: {}", error),
        }
        // unwrap
        let code = result.unwrap();
        println!("Code: {}", code);
        // map
        let mut mode: Result<i32, String> = Ok(4);
        mode = mode.map(|c| c + 1);
        // unwrap_or
        let code = mode.unwrap_or(-1);
        // ------------------------------------
    }
    {
        //* ? operator
        fn error_fn() -> Result<i32, String> {
            Ok(5)
        }

        fn do_stuff(num: i32) -> Result<i32, String> {
            let result = error_fn()?;
            if num == result {
                return Err(format!("{} is equal to {}", num, result));
            }
            Ok(result + num)
        }

        fn main() -> Result<(), String> {
            let code = do_stuff(5)?;
            println!("Code: {}", code);
            Ok(())
        }
        // ------------------------------------
        let x = main();
        println!("{:?}", x);
    }
    {
        //* simple parallel programming
        use std::thread;

        fn main() {
            let value = 5;
            let handle = thread::spawn(move || {
                println!("Hello from a thread!");
                println!("Value: {}", value);
            });
            handle.join().unwrap();
            println!("Finished!");
        }
        // ------------------------------------
        main();
    }
    {
        //* safe parallel programming
        use std::sync::{Arc, Mutex};
        use std::thread;

        fn main() {
            let mut data = Arc::new(Mutex::new(0));
            let mut handles = vec![];
            for x in 0..3 {
                let data_ref = Arc::clone(&data);
                let handle = thread::spawn(move || {
                    let mut data = data_ref.lock().unwrap();
                    *data += 1;
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
            let data: u64 = *data.lock().unwrap();
            println!("{}", data); // 3
        }
        // ------------------------------------
        main();
    }
}
