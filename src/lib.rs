use std::collections::HashMap;

pub struct Solution{}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut letters = HashMap::new();

        for c in magazine.chars() {
            letters.entry(c)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        let mut zero = 0;

        for c in ransom_note.chars() {
            let letter = letters.get_mut(&c).or_else(|| Option::from(&mut zero)).unwrap();
            if *letter > 0 {
                *letter -= 1
            } else {
                return false
            }
        }

        true
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
