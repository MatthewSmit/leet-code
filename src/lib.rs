pub struct Solution{}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..n + 1).map(|i| {
            if i % 3 == 0 && i % 5 == 0 {
                return "FizzBuzz".to_string();
            }
            if i % 3 == 0 {
                return "Fizz".to_string();
            }
            if i % 5 == 0 {
                return "Buzz".to_string();
            }
            i.to_string()
        }).collect()
    }
}
