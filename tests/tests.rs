use leet_code::*;

#[test]
fn example1() {
    let result = Solution::fizz_buzz(3);
    let expected: Vec<String> = ["1","2","Fizz"]
        .iter().map(|x| x.to_string()).collect();
    assert_eq!(result, expected);
}

#[test]
fn example2() {
    let result = Solution::fizz_buzz(5);
    let expected: Vec<String> = ["1","2","Fizz","4","Buzz"]
        .iter().map(|x| x.to_string()).collect();
    assert_eq!(result, expected);
}

#[test]
fn example3() {
    let result = Solution::fizz_buzz(15);
    let expected: Vec<String> = ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]
        .iter().map(|x| x.to_string()).collect();
    assert_eq!(result, expected);
}