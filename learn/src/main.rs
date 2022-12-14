// Functions 

/*
fn another_function() {
    println!("Another function.");
}
*/

/* parameter */

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/* Functions with Return Values */

/*

Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->). 
In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. 
You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.

*/

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // if we place a semicolon at the end of the line containing x + 1, changing it from an expression to a statement, we’ll get an error.
}


fn main() {

   //another_function();
   another_function(44);
   print_labeled_measurement(5, 'h');
   let x = five();


    println!("Hello, world!");

    // define constant
    const ID: i32 = 001;

    let name = "gotam gorabh";
    let mut age = 22;
    //println!("My name is {} and age is {}", name , age);



    // Variable and Mutability
    let x =5;
    println!("The value of x is : {}", x);
    //x = 6;   ---> gives error
    let mut y = 7;
    //y = 6;  ---> it's possible



    // constants 
    const pi:u32 = 78*23;



    // Shadowing 
    let var = 5;
    let var = var + 1;
    {
        let var = var*2;
        println!("The value of var(inner) is: {}",var);
    }
    println!("The value of var is: {}", var);

    /*
    
    The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, 
    we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between 
    some text by inputting space characters, and then we want to store that input as a number:
    
    */
    
    
    // doesn't give error
    let spaces = "   ";
    let spaces = spaces.len();
    
    
    // give error
    /*
    let mut spaces = "   ";
    spaces = spaces.len();
    */
    
    
    //  Data Types in Rust
    
    /*
            **Integer Types**
            
            
            8-bit	i8	u8
            16-bit	i16	u16
            32-bit	i32	u32
            64-bit	i64	u64
            128-bit	i128	u128
            arch	isize	usize
            
      
      
           **Floating-Point Types**
           
           Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. 
           The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision. 
           All floating-point types are signed.
           
           
           
           **The Boolean Type**
           
           let t = true;
           let f: bool = false; // with explicit type annotation
           
           
           **Compound Types**
           
           Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
           
           
                1. The Tuple Type
                   A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
                   Tuples have a fixed length: once declared, they cannot grow or shrink in size.
                   
                   We create a tuple by writing a comma-separated list of values inside parentheses. 
                   Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same. 
                   We’ve added optional type annotations in this example:
                   
                   let tup: (i32, f64, u8) = (500, 6.4, 1);
                   let (x, y, z) = tup;
                   println!("The value of y is: {y}");
                   
                   We can also access a tuple element directly by using a period (.) 
                   followed by the index of the value we want to access. For example:
                   
                   let x: (i32, f64, u8) = (500, 6.4, 1);
                   let five_hundred = x.0;
                   let six_point_four = x.1;
                   let one = x.2;
                   
                2. The Array Type
                   Another way to have a collection of multiple values is with an array. 
                   Unlike a tuple, every element of an array must have the same type. 
                   Unlike arrays in some other languages, arrays in Rust have a fixed length
                   
                   let a = [1, 2, 3, 4, 5];
                   let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
                   
                   let a: [i32; 5] = [1, 2, 3, 4, 5];
                   
                   You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, 
                   and then the length of the array in square brackets, as shown here:
                   
                   let a = [3; 5];
                   
                   Accessing Array Elements
                   
                   let a = [1, 2, 3, 4, 5];
                   let first = a[0];
                   let second = a[1];





    
    */
    
    let a = [1, 2, 3, 4, 5];
    println!("sixth index {}", a[4]);
    
     
    /* Statements and Expressions */
    
    let x = (let y = 6);  // this is wrong because let y = 6 statement does not return a value, 
    
    
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}"); 
    
    /*
    
    This expression:

{
    let x = 3;
    x + 1
}
is a block that, in this case, evaluates to 4. 
That value gets bound to y as part of the let statement. Note that the x + 1 line doesn’t have a semicolon at the end, 
unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. 
If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. 
Keep this in mind as you explore function return values and expressions next
    
    */
    
    
    
    
    
    
    /* control flow in Rust Programming */
    
    // 1. if Expressions
    
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
    /*
      let number = 3;

      if number {
        println!("number was three");
      }
      The if condition evaluates to a value of 3 this time, and Rust throws an error:
    */
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    
    // Using if in a let Statement
    let condition = true;
    let num = if condition { 5 } else { 6 };

    println!("The value of number is: {num}");
    
    /*
    This means the values that have the potential to be results from each arm of the if must be the same type; in Listing 3-2, 
    the results of both the if arm and the else arm were i32 integers. If the types are mismatched, as in the following example, we’ll get an error:
    */
    
    let condition = true;

    let number = if condition { 5 } else { "six" }; // this statement will give error

    println!("The value of number is: {number}");
    
    
    // 2. Repetition with Loops
    
    /*
    
    Rust has three kinds of loops: loop, while, and for.
    */
    
    loop {
        println!("again!");
    }
    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    
    // multilayer loop 
    
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    
    
    // while loop 
    
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    
    
    // for loop 
    
    for element in a {
        println!("the value is: {element}");
    }
    
    
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    
    
    
    
    
    
    
    //  Understanding Ownership  //
    
    /*

    Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks.
    If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

       ** ownership rules **
       1. Each value in Rust has an owner.
       2. There can only be one owner at a time.
       3. When the owner goes out of scope, the value will be dropped.



    */

}

     
     
     

