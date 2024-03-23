
fn longest_word<'a>(x: &'a str , y: &'a str) -> &'a str {
    if x > y {
        x 
    } else { 
        y
    }
}

fn main(){
    let x = String::from("abcd");
    let y = "xyz";

    let result = longest_word(x.as_str(), y);
    println!("The result is {}", result);
} 