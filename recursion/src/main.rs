fn main() {
    dbg!(factorial(3));
}

fn factorial(num: usize) -> usize {
    match num {
        1 => 1,
        _ => num * factorial(num - 1),
    }
}
