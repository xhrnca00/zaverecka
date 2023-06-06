#[allow(dead_code, unused_variables, unused_mut)]

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
}
