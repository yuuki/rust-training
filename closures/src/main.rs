fn factory() -> Box<Fn(i32) -> i32> {
    let num = 5;

    Box::new(move |x| x + num)
}

fn main() {
    let plus_one = |x: i32| x + 1;

    assert_eq!(2, plus_one(1));

    let plus_two = |x| {
        let mut result: i32 = x;
    
        result += 1;
        result += 1;

        result
    };

    assert_eq!(4, plus_two(2));

    let num = 5;
    let plus_num = |x: i32| x + num;

    assert_eq!(10, plus_num(5));

    let f = factory();

    let answer = f(1);
    assert_eq!(6, answer);
}
