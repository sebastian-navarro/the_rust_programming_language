fn main() {
    // Exist two types of data SCALAR TYPE and Compound TYPE

    // SCALAR TYPE
    // Integer, Float, Boolean , Character and usize (not sure) 
    // usize depend from the processor 32 bit or 64 bit the size

    // Integer
    // Exist unsigned 'u' and signed 'i'

    let _a = 8u8; // u8 value have (2 ^ n) - 1 = (2 ^ 8) -1 values that means a value between 0 and 255
    let _b: i8 = -7;  // i8 value have value beetwen - 2 ^ 8-1 and 2 ^ 8-1 -1 that means beetwen - 128 and 127
    let _c: u32 = 300; // u32 value have  (2 ^ 32) - 1 values.
    let _d: u64 = 30000000;
    let _e: u128 = 300_000_000_000; // Another way to separator number '_'
    // Whats up if we superate the range of values

    //let _a = 256_u8; // Compilation time error: literal out of range for `u8`
    // release mode the value is reset and begin in 0 (after 255 is 0)

    // Float number (f32 and f64)

    let _f = 32.0;
    let _g:f32 = -74.15161710;

    // Boolean number (true and false)

    let _t = true;
    let _f: bool = false; // with explicit type annotation
    
    // Char type
    // Store more than ascii characters , UTF-TYPES

    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // Compound type (Multiple values in one type)
    // Tuple and Array
    
    // Tuples
    //  Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    let tuple = ('z',2); // You can store two diferent types in one variable.
    let _complex_tuple = ([1,2,3,4], ['a','b','c']);
    let _empty_tuple = ();

    println!("First value of tuple is {}", tuple.0);


    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    let mut _tuple = ('a', 5);
    //_tuple.0 = 1; Output error mismatched types expected `char`, found `u8`

    

    // Array
    // Every element of an array must have the same type

    let array = [3;5]; // Five times the value 3
    let _array2 = [1,2,3,4,5];

    println!("Values of array: {:?}", array); 

    // If you are sure about the amount , use array instead vector
    // Array is store in stack, Vector is store in the heap

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let _first = months[0];
}