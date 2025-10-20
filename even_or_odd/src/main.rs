fn is_even(n: i32) -> bool {
    if n == 0 {
        true
    }
    else if n == 1 {
        false
    }
    else if n % 2 == 0 {
        true
    } 
    else {
        false
    }
}

fn main() {
    for n in 0..10 {
        println!("{}\t{}", n, is_even(n));
    }
}