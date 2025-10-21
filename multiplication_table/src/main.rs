fn multiplication_table(){
    for i in 1..=10 {
        for j in 1..=10 {
            println!("{} x {} = {}", i, j, i * j);
        }
        println!();
    }
}

fn main() {
    multiplication_table();
}
