use std::io; // importing io from standard library available in rust https://doc.rust-lang.org/std/
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("WELCOME TO Guessing GAME !!"); // println! is macro 
    let secret_number=rand::thread_rng().gen_range(1..=100);
    // Generating random number from 1 to 100 from the rand create we are using in cargo toml file 

    loop { // looping the following to able to give user multiple chance to win the game
        let mut guess=String::new(); // Creating new empty string. ::new() is associate method available in String type
        println!("Enter your number");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input"); 
        // using read_line method of stdin() and passing mutable reference of guess variable to save user input
        // read_line return RESULT which is enum with OK and Err as variables. To wrap the err we use expect clause in Rust
       
        let guess:u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue
        };
        // Shadowing the previous guess variable to parse its value and save integer data in new guess variable
        
       
        match guess.cmp(&secret_number){ //Starting the arm for match statement
         Ordering::Equal=>{
            println!("Congratulations you have won!!"); //if equals return this and loop breaks
            break; //Breaking the loop
         },
         Ordering::Greater=>println!("Too big! Try AGAIN!"), // if greater return this and loop continues
         Ordering::Less=>println!("Too Small! Try AGAIN"), // if small return this and loop continues
        }
    }
}
