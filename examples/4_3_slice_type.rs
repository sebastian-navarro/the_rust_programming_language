
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); //each character is transformed at byte code.
    println!("{:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main(){
    let s = String::from("Hello world");
    let result = first_word(&s);

    println!("The result is : {}", result);

    // is tricky get the word, that's why exist slice
    let _hello = &s[0..5];
    let _world = &s[6..11];

    // other slices 

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}