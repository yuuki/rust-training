fn main() {
    for i in 1..101 {
        if i % 15 == 0 {
            println!("Fizz Buzz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", i);
        }
    }
}
