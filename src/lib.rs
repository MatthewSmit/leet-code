pub struct Solution{}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
