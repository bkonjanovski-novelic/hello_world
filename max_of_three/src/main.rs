fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    if a >= b && a >= c {
        a
    } else if b >= a && b >= c {
        b
    } else {
        c
    }
}

fn main() {
    println!("Max of (3, 7, 5) is {}", max_of_three(3, 7, 5));
}
