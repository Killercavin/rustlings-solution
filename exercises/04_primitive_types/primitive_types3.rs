fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???

    let a = [0; 1000]; // An array with 100 elements.

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
