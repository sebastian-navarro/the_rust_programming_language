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
    // Store more than ascii characters, UTF-TYPES

    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // Compound type (Store more than 1 value)

    // Tuple

    // Array
}