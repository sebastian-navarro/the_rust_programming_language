
fn main(){
    // Shadowing happen when define two times a variable with the same name and the word let
    // Example 1
    let x = 5;

    let x = x + 1; //Shadowing x , with let
    {
        let x = x * 2; // Shadowing x, using let, and using the same name of value 'x'
        println!("The value of x is : {}", x); // After the println, x is droped, because finish the scope { ... }
    }

    println!("The value of x is : {}", x)
    // Output : x = 12 ; x = 6

    // Example 2
    // Shadowing doesnt work if we define the variable mut
    /*
    let mut spaces = "    ";
    spaces = spaces.len();
    */
    // Output Error : mismatched typesexpected `&str`, found `usize`

    // Example 3
    // We can not mutate the variable without the let word, because is inmutable
    /*
    let y = 6;
    y = y + 1;
     */
    
    // Output Error : cannot mutate immutable variable `y`
}