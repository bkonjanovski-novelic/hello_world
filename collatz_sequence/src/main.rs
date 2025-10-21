fn collatz_sequence(n: u32) -> u32 {
    let mut steps = 0;
    let mut i = n;

    while i != 1 { 
        if i % 2 == 0 {
            i = i/2;
        } else {
            i = 3 * i + 1;
        }
        println!("{}", i);
        steps += 1;
    }
    steps
}

fn main() {
    println!("Collatz sequence starting from 6 requires: {} steps ", collatz_sequence(6));
}
