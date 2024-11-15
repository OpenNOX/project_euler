/// Solution to [Multiples of 3 or 5](https://projecteuler.net/problem=1). (Problem 1)
fn main() {
    let mut answer = 0;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            answer += i
        }
    }

    println!("Answer: {}", answer);
}
