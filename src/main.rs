fn main() {
    let s = Some('c');

    // match s {
    //     Some(i) => println!("Value inside match: {}", i),
    //     None => {},
    // }

    if let Some(i) = s {
        println!("Value inside if: {}", i);
    } else {
        println!("Inside else");
    }
}