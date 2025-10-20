fn main() {
    // Store three numbers in a tuple
    let numbers = (10, 20, 30);

    // Destructure the tuple
    let (a, b, c) = numbers;

    // Calculate sum, average, and product
    let sum = a + b + c;
    let average = sum as f32 / 3.0; // Use f32 to get a floating-point average
    let product = a * b * c;

    println!("The numbers are: {}, {}, {}", a, b, c);
    println!("Sum: {}", sum);
    println!("Average: {}", average);
    println!("Product: {}", product);
}
