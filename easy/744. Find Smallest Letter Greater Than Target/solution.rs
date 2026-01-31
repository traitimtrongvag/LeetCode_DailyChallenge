impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let num_letters = letters.len();
        if num_letters > 100 {
            Solution::search_next_greatest_letter(&letters, target)
        } else {
            Solution::scan_next_greatest_letter(&letters, target)
        }
    }

    fn search_next_greatest_letter(letters: &Vec<char>, target: char) -> char {
        let (mut left, mut right) = (0, letters.len());
        while left < right {
            let mid = left + (right - left) / 2;
            if letters[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if left < letters.len() {
            letters[left]
        } else {
            letters[0]
        }
    }

    fn scan_next_greatest_letter(letters: &Vec<char>, target: char) -> char {
        for (i, c) in letters.iter().enumerate() {
            if *c > target {
                return letters[i];
            }
        }
        letters[0]
    }
}