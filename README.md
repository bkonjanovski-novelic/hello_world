# Week 3: Functions and Control Flow

# Exercises

---

## Exercise 1: Even or Odd

Write a function `is_even` that takes an integer and returns true if it's even, false if it's odd. Test it with numbers from 1 to 10.

```rust
fn is_even(n: i32) -> bool {
    // Your code here
}

fn main() {
    // Test your function
}
```

## Exercise 2: Maximum of Three

Write a function that takes three integers and returns the largest one. (Don't use built-in max functions, instead use manual comparison)

```rust
fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    // Your code here
}
```

## Exercise 3: Multiplication Table

Write a program that prints a multiplication table from 1 to 10 using nested loops.

Expected output:
```
1 x 1 = 1
1 x 2 = 2
...
10 x 10 = 100
```

## Exercise 4: Sum of Even Numbers

Write a function that calculates the sum of all even numbers between 1 and n (inclusive).

```rust
fn sum_of_evens(n: u32) -> u32 {
    // Your code here
}

fn main() {
    println!("Sum of evens up to 20: {}", sum_of_evens(20));
}
```

## Exercise 5: FizzBuzz

This is one of very common interview questions, and although it looks very simple, there is really lots of different ways you can solve this exercise.

Write a program that prints the numbers from 1 to 100. But:
- For multiples of 3, print "Fizz" instead of the number
- For multiples of 5, print "Buzz" instead
- For multiples of both 3 and 5, print "FizzBuzz"
