#[derive(Debug)]
enum Types {
    First(String),
    Second(i8),
    Third(char),
}

fn main(){
    let mut vec = vec![1,2,3];
    vec.push(4);
    vec.push(5);
    vec.pop();

    println!("The vector is {:?}", vec);

    let vec2 = vec![Types::First(String::from("hola")),Types::Second(12),Types::Third('@')];

    println!("Vector with diferents types {:?}", vec2);

    let first = &vec2[0];
    let second = vec2.get(1);
    println!("Extract the value from the enum First : {:?}", first);
    println!("Extract the value from the enum Second : {:?}", second);

    for v in &vec {
        println!("For values in v: {}", v);
    }

    for v in &mut vec {
        *v += 50;
    }

    println!("The vector now is modified by : {:?}", vec);


}