use leet_code::*;

#[test]
fn example1() {
    let result = Solution::can_construct("a".to_string(), "b".to_string());
    assert_eq!(result, false);
}

#[test]
fn example2() {
    let result = Solution::can_construct("aa".to_string(), "ab".to_string());
    assert_eq!(result, false);
}

#[test]
fn example3() {
    let result = Solution::can_construct("aa".to_string(), "aab".to_string());
    assert_eq!(result, true);
}